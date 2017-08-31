# easytiming

Easy timing for functions, code blocks etc in Rust

## Quick start

Really quick way to start using `easytiming` is like this:

```rust
extern crate easytiming;

use easytiming::Timing;

fn do_something() {
    let _t = Timing::new("do_something() function");
    ...
}
```

This will produce the next output when run:
```
"do_something() function" was running for 410420 ns
```
