# Getters2

Getters2 is the *real* best "auto-getters"/"auto-setters" crate for Rust.

* Selectable immutable/mutable/clone/deref getters
* Support for named, tuple, and newtype structs
* Support for named, tuple, and newtype *enums*

- [Getters2](#getters2)
  - [Installation](#installation)
  - [Examples](#examples)
    - [Structs](#structs)
    - [Enums](#enums)
  - [Prior Art](#prior-art)

## Installation

```sh
cargo add getters2
```

or, add it to `Cargo.toml`

```toml
[dependencies]
getters2 = "0.1.0"
```


## Examples

Getters2 has a dead simple API.

### Structs

```rust
use getters2::Getters;
#[derive(Getters)]
#[getters(mutable, clone, deref)]
struct Vector3 {
   x: f32,
   y: f32,
   z: f32,
   #[getters(skip, skip_mutable, skip_clone, skip_deref)]
   name: String,
}

let mut v = Vector3 { x: 1.0, y: 2.0, z: 3.0, name: "MyVec".to_string() };
assert_eq!(v.x_ref(), &1.0);
assert_eq!(v.y_ref(), &2.0);
assert_eq!(v.z_ref(), &3.0);
assert_eq!(v.x_deref(), 1.0);
assert_eq!(v.y_deref(), 2.0);
assert_eq!(v.z_deref(), 3.0);
assert_eq!(v.x_clone(), 1.0);
assert_eq!(v.y_clone(), 2.0);
assert_eq!(v.z_clone(), 3.0);
*v.x_mut() = 4.0;
*v.y_mut() = 5.0;
*v.z_mut() = 6.0;
assert_eq!(v.x_ref(), &4.0);
assert_eq!(v.y_ref(), &5.0);
assert_eq!(v.z_ref(), &6.0);
```

### Enums

```rust
use getters2::Getters;
#[derive(Getters)]
#[getters(deref, clone, mutable)]
enum Animal {
   Dog { name: String, age: u8 },
   Cat { name: String, age: u8 },
}
let mut dog = Animal::Dog { name: "Rover".to_string(), age: 5 };
let mut cat = Animal::Cat { name: "Mittens".to_string(), age: 3 };
assert_eq!(dog.dog_name_ref(), Some(&"Rover".to_string()));
assert_eq!(dog.dog_name_deref(), Some("Rover".to_string()));
assert_eq!(dog.dog_name_clone(), Some("Rover".to_string()));
assert_eq!(dog.dog_age_ref(), Some(&5));
assert_eq!(dog.dog_age_deref(), Some(5));
assert_eq!(dog.dog_age_clone(), Some(5));

let Some(dog_name) = dog.dog_name_mut() else {
   panic!("Expected Some");
};
*dog_name = "Spot".to_string();

assert_eq!(dog.dog_name_ref(), Some(&"Spot".to_string()));

assert_eq!(cat.cat_name_ref(), Some(&"Mittens".to_string()));
assert_eq!(cat.cat_name_deref(), Some("Mittens".to_string()));
assert_eq!(cat.cat_name_clone(), Some("Mittens".to_string()));
assert_eq!(cat.cat_age_ref(), Some(&3));
assert_eq!(cat.cat_age_deref(), Some(3));
assert_eq!(cat.cat_age_clone(), Some(3));

let Some(cat_name) = cat.cat_name_mut() else {
  panic!("Expected Some");
};
*cat_name = "Whiskers".to_string();

assert_eq!(cat.cat_name_ref(), Some(&"Whiskers".to_string()));
```


## Prior Art

There are a lot of crates that claim to do this. Getters2 has something they all don't.

* [accessors](https://crates.io/crates/accessors) - Unmaintained
* [accessors-rs](https://crates.io/crates/accessors-rs) - Doesn't support Enums
* [getters](https://crates.io/crates/getters) - Yanked
* [derive-getters](https://crates.io/crates/derive-getters) - Doesn't support enums