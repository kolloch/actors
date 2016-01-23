extern crate env_logger;

use {ActorSpawner};

use tests::{CountingActor, CountingMessage, ForwardingActor, ForwardMessage};

use thread::{DedicatedThreadSpawner};
use channel::channel_actor_ref;

#[test]
fn test_counting_actor_with_dedicated_thread() {
	let counting_actor = CountingActor { count: 0 };
	let spawner = DedicatedThreadSpawner;
	let actor_ref = spawner.spawn(counting_actor);

	actor_ref.send(CountingMessage::Count).unwrap();
	actor_ref.send(CountingMessage::Count).unwrap();
	actor_ref.send(CountingMessage::Count).unwrap();

	let (channel_ref, receive) = channel_actor_ref();
	actor_ref.send(CountingMessage::GetCount(channel_ref)).unwrap();
	let count = receive.recv().unwrap();
	assert_eq!(count, 3)
}

#[test]
fn test_actor_ref() {
	env_logger::init().expect("init logging");

	let counting_actor = CountingActor { count: 0 };
	let spawner = DedicatedThreadSpawner;
	let count_cell = spawner.spawn(counting_actor);

	println!("1");

	let forwarding_actor = ForwardingActor;
	let forwarding_cell = spawner.spawn(forwarding_actor);
	let forwarding_ref = forwarding_cell.clone();

	println!("2");

	let count_ref = count_cell.clone();
	forwarding_ref
		.send(ForwardMessage {forward_to: count_ref.clone(), message: CountingMessage::Count })
		.ok().expect("send1 failed");
	forwarding_ref
		.send(ForwardMessage {forward_to: count_ref.clone(), message: CountingMessage::Count })
		.ok().expect("send2 failed");
	forwarding_ref
		.send(ForwardMessage {forward_to: count_ref.clone(), message: CountingMessage::Count })
		.ok().expect("send3 failed");

	println!("3");

	let (channel_ref, receive) = channel_actor_ref();
	forwarding_ref
		.send(ForwardMessage {forward_to: count_ref.clone(), message: CountingMessage::GetCount(channel_ref) })
		.expect("send 4 failed");

	let count = receive.recv().expect("waiting for count failed");
	assert_eq!(count, 3)
}
