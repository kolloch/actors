//! Channel-based actor-ref implementations

use std::thread;
use std::sync::mpsc::{channel, Sender};

use std::sync::{Arc, Mutex, Condvar};

use Actor;
use ActorRef;

#[cfg(test)]
mod tests;

/// A simplistic environment to run an actor in
/// which can act as ActorRef.
///
/// Currently, it still uses one thread per actor.
pub struct ActorCell<Message, A: Actor<Message>> {
	tx: Mutex<Sender<Option<Message>>>,
	actor: Mutex<Option<A>>,
	actor_var_change: Condvar
}

impl<Message: Send + 'static, A: 'static + Actor<Message>> ActorCell<Message, A> {
	/// Create and ActorCell for the given actor.
	pub fn create(actor: A) -> Arc<ActorCell<Message, A>>
	{
		let (tx, rx) = channel();

		let actor_lock = Mutex::new(Some(actor));
		let actor_var_change = Condvar::new();

		let ret_cell = Arc::new(ActorCell {
			tx: Mutex::new(tx), 
			actor: actor_lock, 
			actor_var_change: actor_var_change
		});

		let cell = ret_cell.clone();

		thread::spawn( move|| {
			let mut actor = {
				cell.actor.lock().unwrap().take().unwrap()
			};
			while let Some(msg) = rx.recv().unwrap() {
				actor.process(msg);
			};
			let mut actor_var = cell.actor.lock().unwrap();
			*actor_var = Some(actor);
			cell.actor_var_change.notify_all()
		});

		ret_cell
	}

	/// Stops the actor cell and returns the latest actor state.	
	pub fn stop_and_join(&self) -> A {
		{
			self.tx.lock().unwrap().send(None).unwrap();
		}

		let mut actor = self.actor.lock().unwrap();
		while actor.is_none() {
			actor = self.actor_var_change.wait(actor).unwrap();
		}

		actor.take().unwrap()
	}
}

impl<Message: Send + 'static, A: 'static + Actor<Message>> ActorRef<Message> for ActorCell<Message, A> {
	fn send(&self, msg: Message) {
		self.tx.lock().unwrap().send(Some(msg)).unwrap();
	}
}
