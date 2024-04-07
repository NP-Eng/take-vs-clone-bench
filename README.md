# Simple benchmarks: `clone` vs. `take`

Suppose we have this struct:

```rust
pub struct MyStruct {
    a: Vec<u64>,
    b: String,
}
```



And we have a function which takes ownership of this struct, does something to its inner vector, and wraps it in a new struct:
```rust
pub struct Wrapper {
    pub a: Vec<u64>,
}


pub fn my_function(s: MyStruct) -> Wrapper {
    // Can't move out of s
    // let a = s.a; // cannot do this

    // Also cannot destructure, as its fields are private
    // let MyStruct { a, .. } = s; // cannot do this

    // Instead need to clone
    let a = s.a.clone();
    Wrapper { a }
}
```

Notice that we no longer need `MyStruct` after we access its field `a`. 
A more efficient approach is to `take` the vector `a` and leave the rest of the struct behind. Behind the scenes, `take` replaces the data with `Default::default()`, which in this case is an empty vector. This makes the operation very cheap.

```rust
pub fn my_function_take(s: MyStruct) -> Wrapper {
    let mut a = s.a;
    let a = std::mem::take(&mut a);
    Wrapper { a }
}
```


Results of `clone` vs. `take` for ~1M field elements (the field chosen here is internally represented as 6 `u64` values):

```
Running benches/my_benchmark.rs (target/release/deps/my_benchmark-e7768fe1a33773cd)
clone_vector            time:   [3.0223 ms 3.0700 ms 3.1220 ms]
Found 13 outliers among 100 measurements (13.00%)
  11 (11.00%) high mild
  2 (2.00%) high severe

take_vector             time:   [799.98 ps 803.61 ps 807.94 ps]
Found 3 outliers among 100 measurements (3.00%)
  3 (3.00%) high mild
```