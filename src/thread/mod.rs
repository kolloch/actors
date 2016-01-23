//! Dedicated single thread actor-ref implementations

use std::thread;
use std::sync::mpsc::{channel, Sender};
use std::sync::mpsc;

use std::sync::{Arc, Mutex};
use std::convert::From;

use {Actor, ActorSpawner};
use ActorRef;
use {SendError, SendErrorReason};

#[cfg(test)]
mod tests;

enum ActorCellMessage<Message> where Message: Send {
	Process(Message),
	StopAndNotify
}

/// A simplistic environment to run an actor in
/// which can act as ActorRef.
///
/// Currently, it still uses one thread per actor.
struct ActorCell<Message, A: Actor<Message>> where Message: Send {
	// TODO: clone sender instead of mutex
	tx: Mutex<Sender<ActorCellMessage<Message>>>,
	actor: Mutex<Option<A>>
}

impl<Message, A: Actor<Message>> Drop for ActorCell<Message, A> where Message: Send {
	fn drop(&mut self) {
		// FIXME: Is it clever that we might panic in Drop?
		self.tx.lock().unwrap().send(ActorCellMessage::StopAndNotify).unwrap();
	}
}

/// An ActorSpawner which spawns a dedicated thread for every
/// actor.
pub struct DedicatedThreadSpawner;

impl ActorSpawner for DedicatedThreadSpawner {
	/// Create and ActorCell for the given actor.
	fn spawn<Message, A>(&self, actor: A) -> Arc<ActorRef<Message>>
		where Message: Send + 'static, A: Actor<Message> + 'static
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
					ActorCellMessage::StopAndNotify => {
						break;
					},
				}
			}
		});

		ret_cell
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
