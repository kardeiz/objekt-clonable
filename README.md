# objekt-clonable

Provides a proc_macro attribute wrapper for [objekt](https://docs.rs/objekt/*/objekt/).

# Usage

```rust
use objekt_clonable::*;


#[clonable]
trait MyTrait: Clone {
    fn recite(&self);
}
```

For additional information, see [dtolnay's](https://github.com/dtolnay) [objekt](https://docs.rs/objekt/*/objekt/).

License: MIT