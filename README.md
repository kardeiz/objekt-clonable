# objekt-clonable

Provides a proc_macro attribute wrapper for [dyn-clone](https://docs.rs/dyn-clone/*/dyn_clone/).

# Usage

```rust
use objekt_clonable::*;

#[clonable]
trait MyTrait: Clone {
    fn recite(&self);
}
```

For additional information, see [dtolnay's](https://github.com/dtolnay) [dyn-clone](https://docs.rs/dyn-clone/*/dyn_clone/).

License: MIT
