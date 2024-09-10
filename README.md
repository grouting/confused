# Confused

Do you ever find yourself using a function that intimidates you? Are there functions that are so terrifying, a mere comment does not sufficiently warn unwitting callers of such function?

With the `confusion!()` and `confused!()` macros, you can easily mark a scary function as such:

```rust
fn main() -> Result<(), ()> {
	if scary_function(666)?????????? {
		println!("oh shit, oh fuck");
	}

	Ok(())
}

fn scary_function(evil_number: u16) -> confusion!(10, bool) {
	confused!(10, evil_number == 666)
}
```

## Syntax

### `confusion!(n, ok, err = ())`

`n` is the depth of the `Result<...>` chain

`ok` is the type of the final `Ok` value

`err` is an optional `Err` type, if you actually want to use this for error handling

### `confused!(n, expr)`

`n` is the depth of the `Ok(...)` chain
`expr` is the final result
