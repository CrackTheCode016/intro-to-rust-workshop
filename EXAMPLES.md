# Rust 101 Examples

Below you will find all the examples used in the HackMD slides for your copy-pasting pleasure.

## Rust 101 - Variables

* Immutable by default, must explicitly declare mutability

```rust
let age: u32 = 22;
❌ age += 1;
// A mutable variable (also, note shadowing)
let mut age: u64 = 22;
✅ age += 1;
```

---

## Rust 101 - Collections

* `Vec` = Growable arrays (handy macro)
* `HashMap` = Sets/Dictionary
* `String` = a `&str` you can modify

```rust
let vector = vec![1, 2, 3];
// or
let mut vector: Vec<u32> = Vec::new();
vector.push(1);

let mut hash_map: HashMap<u32, u32> = HashMap::new();
hash_map.insert(1, 50);

let string = String::from("Hey!");
```

---

## Rust 101 - Functions

```rust
// No parameters or return type
fn a_function() {}

// Just parameters
fn print_age(age: u32) {
    // (btw, this is how you print stuff)
    println!("{age}");
}

// Both (note return syntax)
fn increase_age(age: u32) -> u32 {
    age + 1
}
```

---

## Rust 101 - Structs

```rust
struct Person {
    name: String,
    age: u32
}
```

---

## Rust 101 - Extending Structs

```rust
impl Person {
    fn new(name: String, age: u32) -> Self {
        Person { name, age }
    }
    // Other methods.. i.e., "print_person"..
}
```

---

## Rust 101 - Traits

* Define common behavior for an object

```rust
trait Sound {
    fn make_sound(&self);
}

impl Sound for Person {
    fn make_sound(&self) {
        println!("hello, {}", self.name)
    }
}
```

---

## Rust 101 - Common Traits + Derive

* Common traits to print, clone, etc
* #[derive] macro used to avoid boilerplate

```rust
#[derive(Clone, Debug)]
struct Person {
    name: String,
    age: u32
}
// ..
println!("{:?}", person);
// .. or
let new_person = person.clone();
```

---

## Rust 101 - Generics

* Placeholder types
* `Calculator<u32>`
* `Calculator<i32>`
* `Calculator<String>`

```rust
struct Calculator<T> {
    // T is just a placeholder 
    // for a type we put in the future
    result: T
}
```

---

## Rust 101 - Associated Types

* Another type of generic
* Can be used to configure traits

```rust
trait Factory {
    // Product is an associated type
    type Product: Clone + Debug;
    fn make_product() -> Self::Product;
}
// .. later:

type Product = Table;
```

---

## Rust 101 - Trait Bounds

* Bound placeholder types (generics) with traits
* In this case, we could pass in both `Dog` or a `Person` and it doesn't matter

```rust
fn make_lots_of_noise<T: Sound>(thing: T, thing2: T) {
    //.. other stuff
    thing.make_sound();
    //.. more stuff
}
```

---

## Rust 101 - Trait Bounds

* Restrict it only to numbers that "can count"

```rust
struct Calculator<T: CanCount> {
    result: T
}
```

---

## Rust 101 - Macros

* Code the generates code
* Elimiates boilerplate

```rust
println!();
// or...

::std::io::_print(format_args!("\n"));
```