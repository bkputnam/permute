[![Build Status](https://travis-ci.org/bkputnam/permute.svg?branch=master)](https://travis-ci.org/bkputnam/permute)

# permutor
Permutation Iterator for Rust

In your Cargo.toml file:

```toml
[dependencies.permute]
git = "https://github.com/bkputnam/permute.git"
```

Sample rust code:

```rust
extern crate permute;

fn main() {
    
	let things = vec!["foo", "bar", "baz"];

	for permutation in permute::lexicographically(&things) {
		println!("{} {} {}", *permutation[0], *permutation[1], *permutation[2]);
	}
}

```

outputs:

```
foo bar baz
foo baz bar
bar foo baz
bar baz foo
baz foo bar
baz bar foo
```