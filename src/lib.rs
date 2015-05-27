#![warn(missing_docs)]

//! Actor-like concurrency for rust.

extern crate threadpool;

use std::sync::Arc;

/// A handle for passing messages to an actor
/// or another message processing entity.
///
/// All communication between actors should
/// use this interface.
/// 
/// Note: This is actual very similar to 
/// std::sync::mpsc::Sender. Unfortunately,
/// that is not trait but a struct.
pub trait ActorRef<M: Send>: Send {

	/// Send a message to the referenced actor
	/// or message processing entity.
	///
	/// Depending on the type of the actorRef that might or
	/// might not guarantee delivery of the message.
	/// Also, the actor might not be alive anymore.
	fn send(&self, msg: M) -> Result<(), SendError<M>>;

	/// Checks whether sending a message to this actor ref
	/// will likely work right now.
	fn send_error_state(&self) -> Option<SendErrorReason> {None}
}

/// Reason for failed send.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SendErrorReason {
	/// Message cannot be sent because the message
	/// buffer is full.
	Full,
	/// Message cannot be sent because the recipient
	/// is not reachable anymore.
	Unreachable,
	/// Unknown Error
	Unknown,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
/// Error for attempted send.
pub struct SendError<Message>(SendErrorReason, Message);

/// An actor can process messages that are sent
/// to it sequentially. 
pub trait Actor<Message: Send>: Send {
	/// Process one message, update state 
	fn process(&mut self, msg: Message);
}

impl<Arg: Send, Func: FnMut(Arg) + Send> Actor<Arg> for Func {
	fn process(&mut self, msg: Arg) {
		// let ref mut f = self;
		self(msg);
	}
}

/// Anything that can spawn actors.
pub trait ActorSpawner {

	/// Spawns a new actor returning an actor ref for passing messages to it.
	fn spawn<Message,A,R>(actor: A) -> Arc<R>
		where R: ActorRef<Message>, Message: Send + 'static, A: Actor<Message> + 'static;
}

pub mod channel;

#[cfg(test)]
pub mod tests;

trait Factory {
	type T;

	fn create() -> Self::T;
}	

struct SomeFactory;

impl Factory for SomeFactory {
	type T = i32;

	fn create() -> i32 {3}
}

trait GenFactory {
	fn create<'a,T>(&self, t: T) -> &'a ActorRef<T> where T:Send;
}	

struct DeadRef;

impl<T: Send> ActorRef<T> for DeadRef {
	fn send(&self, msg: T) -> Result<(), SendError<T>> {
		Err(SendError(SendErrorReason::Unreachable, msg))
	}
}

struct SomeOptFactory;

impl GenFactory for SomeOptFactory {
	fn create<'a, T>(&self, t: T) -> &'a ActorRef<T> where T: Send {
		&DeadRef
	}
}

#[test]
fn simple_test() {
	let factory = SomeOptFactory;
	let r = factory.create(2);
	r.send(2);
}