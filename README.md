# Simple benchmarks: `clone` vs. `take`

Suppose we have this struct in an external crate:
```rust
pub struct MyStruct {
    a: Vec<u64>,
    pub b: String,
}

impl MyStruct {
    pub fn a(self) -> Vec<Fq> {
        self.a
    }

    pub fn a_ref(&self) -> &Vec<Fq> {
        &self.a
    }

    pub fn a_mut(&mut self) -> &mut Vec<Fq> {
        &mut self.a
    }
}
```



And we have a function which takes ownership of this struct, does something to both its fields, and returns the inner vector:

```rust
pub fn my_function_clone(s: MyStruct) -> Vec<u64> {
    // Can't move out of s:
    // let a = s.a(); // cannot do this

    // Also cannot destructure, as the field `a` we want is private
    // let MyStruct { a, .. } = s; // cannot do this

    // Instead need to clone the entire vector
    let a: Vec<Fq> = s.a_ref().clone();

    // anonymous closure doing something to `b`, which doesn't use `a`.
    let _ = || s.b();

    a
}
```

Notice that we no longer need `MyStruct` after we access return `a`. 
A more efficient approach is to "take" the vector `a` out of the struct and leave the rest. How to do this? Well `std::mem::take` replaces the underlying data with `Default::default()`, which in this case is an empty vector. This makes the operation *extremely* cheap.

```rust
pub fn my_function_take(s: MyStruct) -> Vec<u64> {
    let a = { mem::take(s.a_mut()) };

    // anonymous closure doing something to `b`, which doesn't use `a`.
    let _ = || s.b;

    a
}
```


Results of `clone` vs. `take` for 2^24 field elements (the field chosen here is internally represented as 6 `u64` values):
```
clone_vector            time:   [94.980 ms 96.027 ms 97.145 ms]

take_vector             time:   [272.09 ns 296.69 ns 321.30 ns]
```

The `take` approach is 300,000x faster! Alright, it's true that the size of the cloned vector is rather large, but it is not unreasonable to have vectors of this size in cryptographic applications.

Still, e.g. for a vector with ~1M elements, the cost of cloning is about 2-3ms, compared to a negligible cost of `take`. Paying a few extra milliseconds here and there might seem innocuous, but is actually quite a big deal if you want to optimize your code and provide a competitive implementation.

Full example here: 