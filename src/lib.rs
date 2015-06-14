#![warn(missing_docs)]

//! Actor-like concurrency for rust.

extern crate threadpool;

pub mod channel;
pub mod thread;

#[cfg(test)]
pub mod tests;

use std::sync::Arc;
use std::fmt;


/// A handle for passing messages to an actor
/// or another message processing entity.
///
/// All communication between actors should
/// use this interface.
/// 
/// Note: This is actual very similar to 
/// std::sync::mpsc::Sender. Unfortunately,
/// that is not trait but a struct.
pub trait ActorRef<M: Send>: Send + Sync {

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

#[derive(PartialEq, Eq, Clone, Copy)]
/// Error for attempted send.
pub struct SendError<Message>(SendErrorReason, Message);

impl<Message> fmt::Debug for SendError<Message> {
	fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		formatter.write_fmt(format_args!("SendError({:?}, ...)", self.0))
	}
}

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

/// An ActorSpawner allows spawning actors. It returns a reference counted
/// ActorRef to communicate with the created Actor. If all references to
/// the actor disappear, the thread should be stopped.
pub trait ActorSpawner {
	/// Spawns a new actor returning an actor ref for passing messages to it.
	fn spawn<Message,A>(&self, actor: A) -> Arc<ActorRef<Message>>
		where Message: Send + 'static, A: Actor<Message> + 'static;
}
