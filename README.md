# thread_spawn

Write Rust functions that spawn threads and implicitly return JoinHandles.

```rust
#![feature(plugin, proc_macro)]

extern crate thread_spawn;

use thread_spawn::thread_spawn;

#[thread_spawn]
fn foo(a: u8, b: u8, (c, _): (u8, u8)) -> bool {
    assert!(a + b + c == 5);
    5 - c == b
}

fn main() {
    let mut x = 0;
    let mut y = 1;
    let res = foo(x, y, (4, 0)).join(); // explicit join call

    match res {
        Ok(res) => println!("result: {:?}", res),
        Err(err) => panic!("Thread panicked: {:?}", err),
    }
}
```

## License

MIT or Apache-2.0, at your option.
