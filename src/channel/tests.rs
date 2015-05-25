use ActorRef;

use tests::{CountingActor,ForwardingActor, ForwardMessage};

use channel::ActorCell;

#[test]
fn test_counting_actor_with_channel() {
	let counting_actor = CountingActor { count: 0 };
	let cell = ActorCell::create(counting_actor);

	cell.send(1);
	cell.send(2);
	cell.send(3);

	let counting_actor = cell.stop_and_join();
	assert_eq!(counting_actor.count, 3)
}

#[test]
fn test_actor_ref() {
	let counting_actor = CountingActor { count: 0 };
	let count_cell = ActorCell::create(counting_actor);

	let forwarding_actor = ForwardingActor;
	let forwarding_cell = ActorCell::create(forwarding_actor);
	let forwarding_ref = forwarding_cell.clone();

	{
		let count_ref = count_cell.clone();
		forwarding_ref.send(ForwardMessage {forward_to: count_ref.clone(), message: 1 });
		forwarding_ref.send(ForwardMessage {forward_to: count_ref.clone(), message: 2 });
		forwarding_ref.send(ForwardMessage {forward_to: count_ref.clone(), message: 3 });
	}

	forwarding_cell.stop_and_join();

	let counting_actor = count_cell.stop_and_join();
	assert_eq!(counting_actor.count, 3)
}
