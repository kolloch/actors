//! Use channels as ActorRefs

use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use {ActorRef, SendError, SendErrorReason};

/// An ActorRef which forwards its messages to a channel
pub struct SenderActorRef<Message> {
	sender: Mutex<mpsc::Sender<Message>> 
	// it would be nice if we somehow could copy the sender inside Arc instead
	// of protecting this with a mutex...
}

impl<Message: Send + 'static> ActorRef<Message> for SenderActorRef<Message> {
	fn send(&self, msg: Message) -> Result<(), SendError<Message>> {
		match self.sender.lock() {
			Err(..) => 
				Err(SendError(SendErrorReason::Unreachable, msg)),
			Ok(tx) =>
				tx.send(msg).map_err({|err| 
					match err {
						mpsc::SendError(msg) => 
							SendError(SendErrorReason::Unreachable, msg),
					}
				}),
		}
	}

}

/// Create an ActorRef and a Receiver so that messages sent to the ActorRef
/// can be obtained via the Receiver.
pub fn channel_ref<T>() -> (SenderActorRef<T>, mpsc::Receiver<T>) {
	let (sender, receiver) = mpsc::channel();
	let sender_ref = SenderActorRef { sender: Mutex::new(sender) };
	(sender_ref, receiver)
}

/// Like channel_ref but wrap the actor ref in an Arc for convenience.
pub fn channel_arc_ref<T: Send + 'static>() -> (Arc<ActorRef<T>>, mpsc::Receiver<T>) {
	let (sender_ref, receiver) = channel_ref();
	(Arc::new(sender_ref), receiver)
}