# Update Method

Automata Field has an update method which is called each frame. It's called `tick()`, but it does the same thing.

```rust,ignore
pub fn tick(&mut self) {
	for x in 0..self.width {
		for y in 0..self.height {
			self.field_alternate[y as usize * self.width as usize + x as usize] = 
				next_middle(self.surroundings_for(x, y))
	    }
	}
	mem::swap(&mut self.field, &mut self.field_alternate);
}
```

`tick()` computes the next game state, writing the results to a secondary buffer, then swaps secondary and primary buffers according to the double buffer pattern.
