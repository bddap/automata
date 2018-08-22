# State

Game Programming Pattern's chapter on state teaches us how to use finite state automata to manage the behavior of in-game objects such as player characters or NPCs.

Modeling behaviors as FSMs makes game code less verbose, and makes a much easier to reason about.

The book gives an example of how state machines can save a game from bugs--translated into rust.

```rust,ignore
enum Input {
	PressB,
	PressDown,
	ReleaseDown,
}

...

fn handleInput(&mut self, input: Input) {
	match input {
		PressB => self.jump(),
		PressDown => if !self.isJumping {
			self.setGraphics(Ducking)
		},
		ReleaseDown => self.setGraphics(Standing),
	}
}
```

The bug in the above program occurs when someone presses B in mid-air. The above, non-FSM code will allow jumps even when the player is in the air.

Here is an example of the state pattern in action:

```rust,ignore
enum PlayerState {
	Standing,
	Jumping,
	Ducking,
	Diving
}

...

pub fn handleInput(&mut self, input: Input) {
	self.state = match (self.state, input) {
		(Standing, PressB) => (
			self.velocity.y = 1.0;
			Jumping
		),
		(Standing, PressDown) => Ducking,
		(Ducking, ReleaseDown) => Standing,
		(s, _) => s,
	}
}
```

While the bug is avoidable without a state machine, it's much easier to catch when using the the State pattern.

Automata uses the State pattern to model cell behavior. In fact, cell behavior is completely defined as a single state machine. Automata's state machine is described as a function called `next_middle`. `next_middle` takes the surrounding cell states as input, and returns the next state of the middle cell.

```rust,ignore
pub fn next_middle(surroundings: Surroundings) -> Automata {
    if let Some(next) = surroundings.infliction_requested() {
        return next;
    }

    match surroundings.middle {
		Water(0) => Air(),                           // No water => Air
        Water(wetness) => Water(wetness.max(1) - 1), // Water drains over time
        Redstone(pow) => Redstone(pow.max(1) - 1),   // Unpowered redstone goes dark
        Slug(_) => Slime(),                          // Slugs leave a trail of slime
        a => a,                                      // Everything else stays the same
    }
}
```

Cells may only modify themselves. This limitation reduces race conditions, but imposes a limitation on Automata. Namely, how does one Automata impose a change on it's neighbor? Consider the state:

```
Slug(Direction)
```

We want this slug to crawl over every Automata in it's path. In other words, every state update, the slug needs to turn the automata it faces into a slug, and turn itself into slime. This is where the `infliction_requested()` method comes in. `infliction_requested()` asks each surrounding automata, "Do you want to change me?", if any answer yes, the middle automata accepts the state given.

`infliction_requested()` calls `inflict()` on each surrounding Automata to find out whether a change of state is requested. Here how the slug destroys all in it's path:

```rust,ignore
fn inflict(&self, other: Self, direction: Direction) -> Option<Self> {
	match self {
		...
		Slug(slug_direction) => if slug_direction == direction {
			Some(Slug(slug_direction))
		} else {
			None
		},
		...
	}
}
```

When neighboring Automata request an infliction, one of the inflictions is chosen using a deterministic set of rules. The rules are essentially a ranking system, the requested state with the highest rank is selected. Here's what happens when two slugs collide.

```
fn resolve_infliction(&self, other: Self) -> Self {
	match (*self, other) {
		(Slug(_), Slug(_)) => Slime(),
		...
	}
}
```

They splat, turning into slime.
