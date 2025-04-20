Like kotlin it keyword, translate `it` to closure

# Examples
```rust
#[closure_it::closure_it]
fn main() {
    assert_eq!([0i32, 1, 2].map(it+2), [2, 3, 4]);
    assert_eq!([0i32, -1, 2].map(it.abs()), [0, 1, 2]);
    assert_eq!(Some(2).map_or(3, it*2), 4);
}
```

```rust
#[closure_it::closure_it(this)]
fn main() {
    assert_eq!([0i32, 1, 2].map(this+2), [2, 3, 4]);
    assert_eq!([0i32, -1, 2].map(this.abs()), [0, 1, 2]);
    assert_eq!(Some(2).map_or(3, this*2), 4);
}
