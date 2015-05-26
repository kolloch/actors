use ActorRef;

use tests::{CountingActor,ForwardingActor, ForwardMessage, CapturingActor};

use channel::ActorCell;

#[test]
fn test_counting_actor_with_channel() {
	let counting_actor = CountingActor { count: 0 };
	let cell = ActorCell::create(counting_actor);

	cell.send(1).unwrap();
	cell.send(2).unwrap();
	cell.send(3).unwrap();

	let counting_actor = cell.stop_and_join();
	assert_eq!(counting_actor.count, 3)
}

#[test]
fn test_counting_actor_with_channel_fn() {
	let capturing_actor = CapturingActor { last_message: 0 };
	let capturing_cell = ActorCell::create(capturing_actor);

	let capturing_cell2 = capturing_cell.clone();
	let mut count = 0;
	let cell = ActorCell::create(move|_| { 
		count += 1;
		capturing_cell2.send(count).unwrap();
	});

	cell.send(1).unwrap();
	cell.send(2).unwrap();
	cell.send(3).unwrap();

	cell.stop_and_join();
	let capturing_actor = capturing_cell.stop_and_join();
	assert_eq!(capturing_actor.last_message, 3)
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
		forwarding_ref
			.send(ForwardMessage {forward_to: count_ref.clone(), message: 1 })
			.ok().unwrap();
		forwarding_ref
			.send(ForwardMessage {forward_to: count_ref.clone(), message: 2 })
			.ok().unwrap();
		forwarding_ref
			.send(ForwardMessage {forward_to: count_ref.clone(), message: 3 })
			.ok().unwrap();
	}

	forwarding_cell.stop_and_join();
	let counting_actor = count_cell.stop_and_join();
	assert_eq!(counting_actor.count, 3)
}
