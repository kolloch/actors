#![warn(missing_docs)]

//! Actor-like concurrency for rust.

#[macro_use]
extern crate log;

// #[cfg(feature = "js-spawn")]
// extern crate jobsteal;

#[cfg(feature = "channel")]
pub mod channel;

#[cfg(feature = "thread")]
pub mod thread;

// #[cfg(feature = "js-spawn")]
// pub mod js_spawn;

#[cfg(test)]
pub mod tests;

use std::fmt;

/// Contains all variants of ActoRefs which are then hidden
/// in the pub stuct `ActorRef` as private field.
///
/// With this pattern, we allow stack allocated `ActorRef`s of
/// different types.
#[derive(Clone, Debug)]
enum ActorRefEnum<M: 'static + Send> {
	#[cfg(feature = "channel")]
	Channel(channel::SenderActorRef<M>),
	#[cfg(feature = "thread")]
	DedicatedThread(thread::ActorCell<M>),
}

/// Corresponds to the public interface of ActorRef.
/// `ActorRefEnum` variants can implement this interface to ensure
/// that the forwarding method calls in `ActorRefEnum` are easy to implement.
trait ActorRefImpl<M: 'static + Send>: Send + Clone + fmt::Debug {
	fn send(&self, msg: M) -> Result<(), SendError<M>>;
}

/// A handle for passing messages to an actor
/// or another message processing entity.
///
/// All communication between actors should
/// use this interface.
pub struct ActorRef<M: 'static + Send>(ActorRefEnum<M>);

impl<M: 'static + Send> ActorRef<M> {

	/// Send a message to the referenced actor
	/// or message processing entity.
	///
	/// Depending on the type of the actorRef that might or
	/// might not guarantee delivery of the message.
	/// Also, the actor might not be alive anymore.
	pub fn send(&self, msg: M) -> Result<(), SendError<M>> {
		let &ActorRef(ref variant) = self;
		match variant {
			#[cfg(feature = "channel")]
			&ActorRefEnum::Channel(ref sender) => sender.send(msg),
			#[cfg(feature = "thread")]
			&ActorRefEnum::DedicatedThread(ref cell) => cell.send(msg),
		}
	}
}

impl<Message: Send + 'static> fmt::Debug for ActorRef<Message> {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		let &ActorRef(ref variant) = self;
		match variant {
			#[cfg(feature = "channel")]
			&ActorRefEnum::Channel(ref sender) => write!(f, "ActorRef({:?})", sender),
			#[cfg(feature = "thread")]
			&ActorRefEnum::DedicatedThread(ref cell) => write!(f, "ActorRef({:?})", cell),
		}
	}
}

impl<Message: Send + 'static> Clone for ActorRef<Message> {
	fn clone(&self) -> ActorRef<Message> {
		let &ActorRef(ref variant) = self;
		let variant = match variant {
			#[cfg(feature = "channel")]
			&ActorRefEnum::Channel(ref sender) =>
				ActorRefEnum::Channel(sender.clone()),
			#[cfg(feature = "thread")]
			&ActorRefEnum::DedicatedThread(ref cell) =>
				ActorRefEnum::DedicatedThread(cell.clone()),
		};
		ActorRef(variant)
	}
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
	fn spawn<Message,A>(&self, actor: A) -> ActorRef<Message>
		where Message: Send + 'static, A: Actor<Message> + 'static;
}
