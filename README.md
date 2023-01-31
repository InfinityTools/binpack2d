# binpack2d

`binpack2d` is a library for packing sets of arbitrary 2d rectangles into a larger bin. The algorithms are all
approximations and use various heuristics, since the problem itself is intractable. 

The implementation is based on Jukka Jylänki's original C++ implementation of the RectangleBinPack.

# Quickstart

```
# In your Cargo.toml
[dependencies]
binpack2d = "0.1"
```

```rust
use binpack2d::{bin_new, BinType, Dimension};

fn main() {
    // Create a number of items to be placed into the bin.
    let items_to_place = vec![
        // Items with autogenerated identifiers.
        // Identifiers start at 1 and increment by 1 per call.
        Dimension::new(188, 300),
        Dimension::new(32, 32),
        Dimension::new(420, 512),
        Dimension::new(620, 384),
        // Three more items with explicit identifiers: -1, 300, and 9528 respectively
        Dimension::with_id(-1, 160, 214),
        Dimension::with_id(300, 384, 640),
        Dimension::with_id(9528, 400, 200),
    ];

    // Create a bin with the dimensions 1024x1024, using the "MaxRects" bin type.
    let mut bin = bin_new(BinType::MaxRects, 1024, 1024);

    // Perform the bin packing operation on the list of items.
    let (inserted, rejected) = bin.insert_list(&items_to_place);

    // Let's see if our item with id=9528 was successfully inserted...
    if let Some(rect) = &bin.find_by_id(9528) {
        println!("Item with id {} was placed into the bin at position (x: {}, y: {})",
                 rect.dim().id(), rect.x(), rect.y());
    } else {
        println!("Item with id 9528 could not be placed into the bin.");
    }

    // List all successfully inserted rectangles.
    if !inserted.is_empty() {
        inserted.iter().for_each(|rect| println!("Inserted: {}", rect));
    } else {
        println!("No rectangles were added to the bin.");
    }

    // List all items which could not be inserted into the bin.
    if !rejected.is_empty() {
        rejected.iter().for_each(|item| println!("Rejected: {}", item));
    } else {
        println!("No items were rejected.");
    }

    println!("Occupancy of the bin: {:.1} %", bin.occupancy() * 100.0);
}
```

<!-- The full API Documentation can be found here: https://docs.rs/binpack2d -->
