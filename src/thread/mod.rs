//! Dedicated single thread actor-ref implementations

use std::thread;
use std::sync::mpsc::{channel, Sender};

use std::sync::{Arc, Mutex};
use std::fmt::{self, Debug, Formatter};

use {Actor, ActorSpawner};
use {ActorRef, ActorRefImpl, ActorRefEnum};
use {SendError};

#[cfg(test)]
mod tests;

/// A simplistic environment to run an actor in
/// which can act as ActorRef.
///
/// It uses one thread per actor.
pub struct ActorCell<Message: Send> {
	tx: Sender<Message>,
	actor: Arc<Mutex<Box<Actor<Message>>>>,
}

/// An ActorSpawner which spawns a dedicated thread for every
/// actor.
pub struct DedicatedThreadSpawner;

impl ActorSpawner for DedicatedThreadSpawner {
	/// Create and ActorCell for the given actor.
	fn spawn<Message, A>(&self, actor: A) -> ActorRef<Message>
		where Message: Send + 'static, A: Actor<Message> + 'static
	{
		let (tx, rx) = channel();

		let actor_box: Box<Actor<Message>> = Box::new(actor);
		let actor = Arc::new(Mutex::new(actor_box));
		let actor_for_thread = actor.clone();
		thread::spawn( move|| {
			let mut actor = actor_for_thread.lock().unwrap();

			loop {
				match rx.recv() {
					Ok(msg) => {
						debug!("Processing");
						actor.process(msg);
					},
					Err(error) => {
						debug!("Quitting: {:?}", error);
						break;
					},
				}
			}
		});

		ActorRef(
			ActorRefEnum::DedicatedThread(
				ActorCell {
					tx: tx,
					actor: actor
				}
			)
		)
	}
}

impl<Message: Send + 'static> ActorRefImpl<Message> for ActorCell<Message> {
	fn send(&self, msg: Message) -> Result<(), SendError<Message>> {
		Ok(try!(self.tx.send(msg)))
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
			actor: self.actor.clone(),
		}
	}
}
