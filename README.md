# Gamevecs

Gamevecs is a library that provides 2d and 3d vectors for game developement in rust. The vectors have common functionality like cross/dot product, lerping, etc. . 

# Examples

```rust
use gamevecs::Vec2;

fn main() {
	//create 2 2D vectors
	let vec1 = Vec2::new(0.0, 10.0);
	let vec2 = Vec2::new(10.0, 0.0);
	
	//add them together
	let result = vec1 + vec2;
	
	//print the result (10.0, 10.0)
	println!(result);
}
```

