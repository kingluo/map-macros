# map-macros
Macros to construct collection literal.

```rust
use map_macros::{map, btree_map, set, btree_set};

fn main() {
    let mm = map!{"hello"=>"world", "yes"=>"ok"};
    println!("{:?}", mm);

    let mm = btree_map!{"hello"=>"world", "yes"=>"ok"};
    println!("{:?}", mm);

    let mm = set!{"hello", "yes"};
    println!("{:?}", mm);

    let mm = btree_set!{"hello", "yes"};
    println!("{:?}", mm);
}
```

Note that this library only supports rust 1.53 or later.
