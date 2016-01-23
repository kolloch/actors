//! ActorSpawner which spawns work on jobsteal thread-pool.

use std::sync::mpsc::{channel, Sender, Receiver, TryRecvError};

use std::sync::{Arc, Mutex};
use std::fmt::{self, Debug, Formatter};
use std::sync::atomic::{AtomicBool, Ordering};
use jobsteal::{self, WorkPool};

use {Actor, ActorSpawner};
use {ActorRef, ActorRefImpl, ActorRefEnum};
use {SendError};

#[cfg(test)]
mod tests;

/// An actor cell pointing to an actor running on a work pool.
pub struct ActorCell<Message: Send> {
	tx: Sender<Message>,
	state: Arc<ActorState<Message>>,
}

struct ActorState<Message: Send> {
    pool: Arc<WorkPoolSpawner>,
    scheduled: AtomicBool,
    actionable: Mutex<Actionable<Message>>,
}

struct Actionable<Message: Send> {
	rx: Receiver<Message>,
	actor: Box<Actor<Message>>,
}

pub struct WorkPoolSpawner {
    pool: Mutex<WorkPool>,
    message_batch_size: usize,
}

impl WorkPoolSpawner {
	pub fn new() -> Arc<WorkPoolSpawner> {
		Arc::new(
			WorkPoolSpawner {
				pool: Mutex::new(jobsteal::make_pool(8).unwrap()),
				message_batch_size: 10,
			}
		)
	}
}

impl ActorSpawner for Arc<WorkPoolSpawner> {
	/// Create and ActorCell for the given actor.
	fn spawn<Message, A>(&self, actor: A) -> ActorRef<Message>
		where Message: Send + 'static, A: Actor<Message> + 'static
	{
		let (tx, rx) = channel();

		let actor_box = Box::new(actor) as Box<Actor<Message>>;
		let actionable = Mutex::new(
			Actionable {
				rx: rx,
				actor: actor_box,
			}
		);
        let state = ActorState {
            pool: self.clone(),
            scheduled: AtomicBool::new(false),
            actionable: actionable,
        };

		ActorRef(
			ActorRefEnum::InJobStealPool(
				ActorCell {
					tx: tx,
					state: Arc::new(state),
				}
			)
		)
	}
}

impl<Message: Send + 'static> ActorRefImpl<Message> for ActorCell<Message> {
	fn send(&self, msg: Message) -> Result<(), SendError<Message>> {
		try!(self.tx.send(msg));

		if !self.state.scheduled.swap(true, Ordering::AcqRel) {
			fn submit<Message: Send + 'static>(local_state: &Arc<ActorState<Message>>) {
				let state: Arc<ActorState<Message>> = (*local_state).clone();
				let mut pool = local_state.pool.pool.lock().expect("get access to pool");
		        pool.submit( move|_| {
					let ref mut actionable = state.actionable.lock().unwrap();

					let mut last_receive_got_message = false;
		            for _ in 0..state.pool.message_batch_size {
						last_receive_got_message = false;
		                match actionable.rx.try_recv() {
		                    Ok(msg) => {
		                        debug!("Processing");
		                        actionable.actor.process(msg);
								state.scheduled.swap(false, Ordering::AcqRel);
								last_receive_got_message = true;
		                    },
							Err(TryRecvError::Empty) => {
		                        debug!("no more messages for now");
		                        break;
		                    },
		                    Err(TryRecvError::Disconnected) => {
		                        debug!("Quitting because of disconnect");
		                        break;
		                    },
		                }
		            }
					if last_receive_got_message {
						submit(&state);
					}
		        });
			};
			submit(&self.state);
		}

		Ok(())
	}
}

impl<Message: Send + 'static> Debug for ActorCell<Message> {
	fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
		write!(f, "ActorCell")
	}
}

impl<Message: Send + 'static> Clone for ActorCell<Message> {
	fn clone(&self) -> ActorCell<Message> {
		ActorCell {
			tx: self.tx.clone(),
			state: self.state.clone(),
		}
	}
}
