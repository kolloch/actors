//! Channel-based actor-ref implementations

use std::thread;
use std::sync::mpsc::{channel, Sender};
use std::sync::mpsc;

use std::sync::{Arc, Mutex, Condvar};
use std::convert::From;

use {Actor, ActorSpawner};
use ActorRef;
use {SendError, SendErrorReason};

#[cfg(test)]
mod tests;

impl<Message: Send> ActorRef<Message> for Sender<Message> {
	fn send(&self, message: Message) -> Result<(), SendError<Message>> {
		self.send(message).map_err(SendError::from)
	}
}

enum ActorCellMessage<Message, Actor> {
	Process(Message),
	StopAndNotify(Arc<(Condvar,Mutex<Option<Actor>>)>)
}

/// A simplistic environment to run an actor in
/// which can act as ActorRef.
///
/// Currently, it still uses one thread per actor.
struct ActorCell<Message, A: Actor<Message>> {
	// TODO: clone sender instead of mutex
	tx: Mutex<Sender<ActorCellMessage<Message, A>>>,
	actor: Mutex<Option<A>>
}

pub struct SingleThreadCell;

impl ActorSpawner for SingleThreadCell {
	/// Create and ActorCell for the given actor.
	pub fn spawn<Message, A, R>(actor: A) -> Arc<R>
		where R: ActorRef<Message>, Message: Send + 'static, A: Actor<Message> + 'static
	{
		let (tx, rx) = channel();

		let actor_lock = Mutex::new(Some(actor));

		let ret_cell = Arc::new(ActorCell {
			tx: Mutex::new(tx), 
			actor: actor_lock
		});

		let cell = ret_cell.clone();

		thread::spawn( move|| {
			let mut actor = cell.actor.lock().unwrap().take().unwrap();

			loop {
				match rx.recv().unwrap() {
					ActorCellMessage::Process(msg) => {
						actor.process(msg);
					},
					ActorCellMessage::StopAndNotify(arc) => {
						let (ref condvar, ref actor_in_mesage) = *arc;
						let mut actor_var = actor_in_mesage.lock().unwrap();
						*actor_var = Some(actor);
						condvar.notify_all();
						break;
					},
				}
			}
		});

		ret_cell
	}	
}

impl<Message: Send + 'static, A: 'static + Actor<Message>> ActorCell<Message, A> {

	/// Stops the actor cell and returns the latest actor state.	
	pub fn stop_and_join(&self) -> A {
		let actor_arc = Arc::new((Condvar::new(), Mutex::new(None)));

		{
			self.tx.lock().unwrap().send(ActorCellMessage::StopAndNotify(actor_arc.clone())).unwrap();
		}

		let (ref actor_var_change, ref actor_var) = *actor_arc;
		let mut actor = actor_var.lock().unwrap();
		while actor.is_none() {
			actor = actor_var_change.wait(actor).unwrap();
		}

		actor.take().unwrap()
	}
}

impl<Message> From<mpsc::SendError<Message>> for SendError<Message> {
	fn from(err: mpsc::SendError<Message>) -> SendError<Message> {
		match err {
			mpsc::SendError(message) => SendError(SendErrorReason::Unreachable, message)
		}
	}
}

impl<Message: Send + 'static, A: 'static + Actor<Message>> ActorRef<Message> for ActorCell<Message, A> {
	fn send(&self, msg: Message) -> Result<(), SendError<Message>> {
		match self.tx.lock() {
			Err(..) => 
				Err(SendError(SendErrorReason::Unreachable, msg)),
			Ok(tx) =>
				tx.send(ActorCellMessage::Process(msg)).map_err({|err| 
					match err {
						mpsc::SendError(ActorCellMessage::Process(msg)) => 
							SendError(SendErrorReason::Unreachable, msg),
						_ => unreachable!()
					}
				}),
		}
	}
}
