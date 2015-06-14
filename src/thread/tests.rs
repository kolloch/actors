use {ActorRef, ActorSpawner};

use tests::{CountingActor, CountingMessage, ForwardingActor, ForwardMessage};

use thread::{DedicatedThreadSpawner};
use channel::channel_arc_ref;

#[test]
fn test_counting_actor_with_dedicated_thread() {
	let counting_actor = CountingActor { count: 0 };
	let spawner = DedicatedThreadSpawner;
	let actor_ref = spawner.spawn(counting_actor);

	actor_ref.send(CountingMessage::Count).unwrap();
	actor_ref.send(CountingMessage::Count).unwrap();
	actor_ref.send(CountingMessage::Count).unwrap();

	let (channel_ref, receive) = channel_arc_ref();
	actor_ref.send(CountingMessage::GetCount(channel_ref)).unwrap();
	let count = receive.recv().unwrap();
	assert_eq!(count, 3)
}

#[test]
fn test_actor_ref() {
	let counting_actor = CountingActor { count: 0 };
	let spawner = DedicatedThreadSpawner;
	let count_cell = spawner.spawn(counting_actor);

	let forwarding_actor = ForwardingActor;
	let forwarding_cell = spawner.spawn(forwarding_actor);
	let forwarding_ref = forwarding_cell.clone();

	let count_ref = count_cell.clone();
	forwarding_ref
		.send(ForwardMessage {forward_to: count_ref.clone(), message: CountingMessage::Count })
		.ok().unwrap();
	forwarding_ref
		.send(ForwardMessage {forward_to: count_ref.clone(), message: CountingMessage::Count })
		.ok().unwrap();
	forwarding_ref
		.send(ForwardMessage {forward_to: count_ref.clone(), message: CountingMessage::Count })
		.ok().unwrap();

	let (channel_ref, receive) = channel_arc_ref();
	forwarding_ref
		.send(ForwardMessage {forward_to: count_ref.clone(), message: CountingMessage::GetCount(channel_ref) })
		.ok().unwrap();

	let count = receive.recv().unwrap();
	assert_eq!(count, 3)
}
