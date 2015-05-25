use std::thread;
use std::sync::Arc;

use Actor;
use ActorRef;

#[derive(Debug)]
pub struct CapturingActor<Message> {
	pub last_message: Message,
}

impl<Message: Send> Actor<Message> for CapturingActor<Message> {
	fn process(self: &mut Self, message: Message) {
		self.last_message = message;
	}
}

#[derive(Debug)]
pub struct CountingActor {
	pub count: i32,
}

impl<Message: Send> Actor<Message> for CountingActor {
	fn process(self: &mut Self, _: Message) {
		self.count += 1;
	}
}

#[test]
fn test_counting_actor() {
	let mut counting_actor = CountingActor { count: 0 };
	{
		let handle = thread::spawn( move|| {
			{
				let mut actor: &mut Actor<i32> = &mut counting_actor;
				actor.process(1);
				actor.process(2);
				actor.process(3);
			}
			assert_eq!(counting_actor.count, 3)
		});
		handle.join().unwrap();
	}
}

#[derive(Debug)]
pub struct ForwardMessage<Message: 'static + Send, Ref: ActorRef<Message> + Sized> {
	pub forward_to: Arc<Ref>,
	pub message: Message,
} 

#[derive(Debug)]
pub struct ForwardingActor;

impl<Message: 'static + Send, Ref: ActorRef<Message> + Sized + Sync> Actor<ForwardMessage<Message, Ref>> for ForwardingActor {
	fn process(&mut self, message: ForwardMessage<Message, Ref>) {
		message.forward_to.send(message.message).ok();
	}
}
