//! Use channels as ActorRefs

use std::sync::mpsc;
use std::fmt::{self, Debug, Formatter};
use {ActorRef, ActorRefEnum, ActorRefImpl, SendError};

/// An ActorRef which forwards its messages to a channel
pub struct SenderActorRef<Message>(mpsc::Sender<Message>);

impl<Message: Send + 'static> ActorRefImpl<Message> for SenderActorRef<Message> {
	fn send(&self, msg: Message) -> Result<(), SendError<Message>> {
		let &SenderActorRef(ref tx) = self;
		Ok(try!(tx.send(msg)))
	}
}

impl<Message: Send + 'static> Debug for SenderActorRef<Message> {
	fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
		write!(f, "SenderActorRef")
	}
}

impl<Message: Send + 'static> Clone for SenderActorRef<Message> {
	fn clone(&self) -> SenderActorRef<Message> {
		let &SenderActorRef(ref sender) = self;
		SenderActorRef(sender.clone())
	}
}

/// Create an ActorRef and a Receiver so that messages sent to the ActorRef
/// can be obtained via the Receiver.
pub fn channel_ref<T>() -> (SenderActorRef<T>, mpsc::Receiver<T>) {
	let (sender, receiver) = mpsc::channel();
	let sender_ref = SenderActorRef(sender);
	(sender_ref, receiver)
}

/// Like channel_ref but wrap the actor ref in an ActorRef for convenience.
pub fn channel_actor_ref<T: Send + 'static>() -> (ActorRef<T>, mpsc::Receiver<T>) {
	let (sender_ref, receiver) = channel_ref();
	(ActorRef(ActorRefEnum::Channel(sender_ref)), receiver)
}
