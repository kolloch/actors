use std::thread;
use std::sync::Arc;

use Actor;
use ActorRef;

#[test]
fn test_fn_mut_as_actor() {
	let mut counter = 0;
	{
		let mut func = |_| { 
			counter+=1
		};
		let counter_actor: &mut Actor<i32> = &mut func;
		counter_actor.process(2);
	}
	assert_eq!(counter, 1)
}

pub enum CountingMessage {
	Count,
	GetCount(Arc<ActorRef<i32>>)
}

#[derive(Debug)]
pub struct CountingActor {
	pub count: i32,
}

impl Actor<CountingMessage> for CountingActor {
	fn process(self: &mut Self, msg: CountingMessage) {
		match msg {
			CountingMessage::Count            => { self.count += 1; },
			CountingMessage::GetCount(sender) => { sender.send(self.count).unwrap(); }
		}
		
	}
}

#[test]
fn test_counting_actor() {
	let mut counting_actor = CountingActor { count: 0 };
	{
		let handle = thread::spawn( move|| {
			{
				let mut actor: &mut Actor<CountingMessage> = &mut counting_actor;
				actor.process(CountingMessage::Count);
				actor.process(CountingMessage::Count);
				actor.process(CountingMessage::Count);
			}
			assert_eq!(counting_actor.count, 3)
		});
		handle.join().unwrap();
	}
}

pub struct ForwardMessage<Message: 'static + Send> {
	pub forward_to: Arc<ActorRef<Message>>,
	pub message: Message,
} 

#[derive(Debug)]
pub struct ForwardingActor;

impl<Message: 'static + Send,> Actor<ForwardMessage<Message>> for ForwardingActor {
	fn process(&mut self, message: ForwardMessage<Message>) {
		message.forward_to.send(message.message).ok();
	}
}
