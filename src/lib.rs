#![warn(missing_docs)]

//! Actor-like concurrency for rust.

/// A handle for passing messages to an actor.
///
/// All communication between actors should
/// use this interface.
pub trait ActorRef<Message: Send>: Send {
	/// Send a message to the reference actor.
	///
	/// Depending on the type of the actorRef that might or
	/// might not guarantee delivery of the message.
	/// Also, the actor might not be alive anymore.
	fn send(&self, msg: Message);
}

/// An actor can process messages that are sent
/// to it sequentially. 
pub trait Actor<Message: Send>: Send {
	/// Process one message, update state 
	fn process(&mut self, msg: Message);
}

impl<Message: Send> Actor<Message> for FnMut(Message) + Send {
	fn process(&mut self, msg: Message) {
		self(msg)
	}
}

pub mod channel;

#[cfg(test)]
pub mod tests;