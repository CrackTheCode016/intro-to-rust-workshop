# Slides

The following is a set of slides which you can copy and paste into hackmd to replicate, if you like.

---

# Introduction to Rust 


---

## Workshop Overview

This workshop will primarily focus on introducing those new to Substrate/Polkadot SDK to Rust (but perhaps not new to programming).  It aims to cover the necessary material to _introduce_ Rust tooling, the concepts used in Substrate, and having them run their first lines of Rust code locally on their machine.

> Note: if you want to copy-paste this into a hackmd, I recommend copying from here below, not from the very beginning!

---

## Learning Goals

* Install Rust and its tooling
* Learn the basics of Rust, along with how certain features are useful for blockchain development.
* Learn why Rust is important as a programming language, especially in the context of blockchains.
* Learn how to write some basic Rust
* Go over the importance of generic programming, and what it can do

---

## Workshop Objectives

* By the end of this workshop, you should:
  * Have Rust, cargo, and its associated tooling installed
  * Learn crucial Rust concepts needed to use the Polkadot SDK
  * Run your first lines of Rust code, and be aware of some of its foundational concepts
  * Be exposed to an actual pallet environment, and learn its structure

---

## Install Rust

- https://www.rust-lang.org/tools/install
- To verify, run `cargo --version` and `rustc --version`
- Should take you less than 15 mins!
- Also, for later, run: `cargo install cargo-expand`

---

## Why Rust for blockchains?

* TLDR - Safe and fast code.
* Easily compile to WebAssembly (and other targets)
* A good option for sensitive applications
* Strict and extensible type system (static & strongly typed)
* The Rust compiler yells at you.. a lot (_in a good way!_)

---

## Rust yelling at you: Exhibit 1

- Ownership in Rust - avoiding common mem managment

```rust

fn take_my_value(str: String) {
    println!("{str}");
}

fn main() {
   let my_name = String::from("Bader");
   take_my_value(my_name);
   println!("{my_name}");
}
```

---

## Rust yelling at you: Exhibit 1

```rust
error[E0382]: borrow of moved value: `my_name`
 --> src/main.rs:9:14
  |
7 |    let my_name = String::from("Bader");
  |        ------- move occurs because `my_name` has type `String`, which does not implement the `Copy` trait
8 |    take_my_value(my_name);
  |                  ------- value moved here
9 |    println!("{my_name}");
  |              ^^^^^^^^^ value borrowed here after move
  ...
```

---

## Rust is also versatile!

* Rust can compile to WebAssembly, allowing interop between languages
* Once you know it, it's easy to prototype with
* Bottomline: protects against human error

---

## In Summary:

![GHNuE2kWoAIvA9o](https://hackmd.io/_uploads/SkSy5EFha.jpg)
<sup>https://twitter.com/shuttle_dev</sup>

---

## Rust 101 - Speedrun

We will quickly go over some Rust concepts.
<sup>Note that this is just what's important for us right now, there is obviously a lot more in Rust</sup>

---

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

```rust!
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

---

## Generic Programming

* Now, to put it together
* Generic Programming: MAke assumptions about *behavior*, but not specific *implemention*

---

## Example: Polkadot!

* Doesn't make assumptions about what you implement
* Uses traits and generics to tell you _how to implement_
* Traits used as a _guide_ for how to implement common components (i.e., a block, a transaction, a pallet)

---

## Example: Pallets

* Pallets are building blocks for your blockchain
* Each pallet's configuration is a trait, called `Config`, which has associated types.

---

## Example: Pallets

```rust=
/// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The max amount of messages per user per conversation
        #[pallet::constant]
        type MaxMessageAmount: Get<u32>;
    }
```

---

## Then later...

- Later, all of these are combined into one big runtime, which is essentially the business logic of our blockchain
- But don't worry about that right now!

```rust=
impl pallet_uke::Config for Runtime {
	type MaxMessageAmount = ConstU32<16>;
}
```

---

## Workshop Time: Factory Example
* Our goal: provide a way to create factories for multiple processes of the same kind
* Raw Materials -> Processes -> Products
* Example: IronOre -> Process It -> Steel
* We can also have many processes operating at the same time!

---

## Factory Struct

* The generic `P` process
* We can access associated types from our trait!
* This is only one type of process per factory (homogenous factory)

```rust=
struct Factory<P: Process> {
    factory_id: u32,
    processes: Vec<P>,
    completed_products: HashMap<usize, P::Product>,
}
```

---

## Factory Methods

```rust!
impl<P: Process> Factory<P> {
    // Our "constructor"
    fn new(factory_id: u32) -> Self {
        Factory { factory_id, processes: vec![], completed_products: HashMap::new() }
    }
```

---

## Factory Methods

```rust!
fn process_all(&mut self) {
    // loop thru and run "push_along_belt", then push the products to completed products.
    for (id, process) in self.processes.iter().enumerate() {
        println!("Processing {}...", id);
        let product: <P as Process>::Product = process.push_along_the_belt(process.material());
        self.completed_products.insert(id, product.clone());
        println!("Process successful, product produced! {:?}...", product);
    }
}
```

---

## Factory Methods

```rust!
fn add_new_belt(&mut self, process: P) {
    self.processes.push(process)
}

// Run the factory
fn run(&mut self) {
    // run all belts
    self.process_all();
}
```

---

## Process Trait

* Remember: we want to define _how_ a process works, not just a process itself
* A blueprint for infinitely many processes!
* Note: we can also use trait bounds on associated types.

---

## Process Trait

```rust!
trait Process {
    // Note the use of associated functions as a means to "configure" our belt
    type RawMaterial;
    type Product: Debug + Clone;
    
    /// Processes the item from a raw material to a product.
    fn push_along_the_belt(&self, material: Self::RawMaterial) -> Self::Product;

    // The raw material to process on this belt
    fn material(&self) -> Self::RawMaterial;
}
```

---

## Raw Material: IronOre

```rust!
struct OreBelt {
    amount: u32
}
```

---

## Product: Steel

```rust!
#[derive(Debug, Clone)]
struct Steel {
    amount: u32,
}
```

---

## Process


```rust!
struct OreProcess {
    amount: u32
}
```

---

## Implementing Our Process

```rust!
impl Process for OreProcess {
    type RawMaterial = IronOre;
    type Product = Steel;

    fn push_along_the_belt(&self, material: Self::RawMaterial) -> Self::Product {
        Steel { amount: material.amount / 2 }
    }

    fn material(&self) -> Self::RawMaterial {
        IronOre { amount: self.amount }
    }
}
```

---

## Running our Factory

```rust!
/// Do factory stuff.
fn main() {
    let ore_process = OreProcess { amount: 20 };
    let mut metal_factory: Factory<OreProcess> = Factory::new(0);
    metal_factory.add_new_process(ore_process);
    metal_factory.run();
}
```

---

## Summary

* We defined how a factory process operates via a trait
* We successfully created a new process, and ran our factory!
* Lots of innovation can happen (multithreading for process, multi-process Factory, a Factory trait...)


---

## A Look at a Substrate Pallet

* Usually this is with the node-template, but this specific repo is just the pallet on its own

https://github.com/CrackTheCode016/substrate-pallet-template

---


## Where to go next

* [The Rust Book](https://doc.rust-lang.org/book/ch10-01-syntax.html)
* [Rust State Machine by Shawn Tabrizi](https://www.shawntabrizi.com/rust-state-machine-mdbook/)
* [Substrate Node Template](https://github.com/substrate-developer-hub/substrate-node-template)

