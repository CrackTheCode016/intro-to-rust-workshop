use std::{ collections::HashMap, fmt::Debug };

trait Process {
    // Note the use of associated functions as a means to "configure" our belt
    type RawMaterial;
    type Product: Debug + Clone;
    /// Processes the item from a raw material to a product.
    fn push_along_the_belt(&self, material: Self::RawMaterial) -> Self::Product;

    // The raw material to process on this belt
    fn material(&self) -> Self::RawMaterial;
}

/// What a factory looks like.  Each factory can only process one type of thing, in theory we could
// "parallelize" these. You can think of these belts as parachains on Polkadot, which execute transactions horizontally.
// Note that we wanted a multi-belt factory, we would need to use dynamic dispatch for both processes (Vec<Box<dyn Belt>>)
struct Factory<P: Process> {
    factory_id: u32,
    processes: Vec<P>,
    completed_products: HashMap<usize, P::Product>,
}

impl<P: Process> Factory<P> {
    // Our "constructor"
    fn new(factory_id: u32) -> Self {
        Factory { factory_id, processes: vec![], completed_products: HashMap::new() }
    }

    fn add_new_process(&mut self, process: P) {
        self.processes.push(process)
    }

    fn process_all(&mut self) {
        // loop thru and run "push_along_belt", then push the products to completed products.
        for (id, process) in self.processes.iter().enumerate() {
            println!("Processing {}...", id);
            let product = process.push_along_the_belt(process.material());
            self.completed_products.insert(id, product.clone());
            println!("Process successful, product produced! {:?}...", product);
        }
    }

    // Run the factory
    fn run(&mut self) {
        // run all processes
        self.process_all();
    }
}

// Look at all this yucky boilerplate...
// I wonder if there is a better way to do this (there is - macros!)

// Also, the amount here is in tons
struct IronOre {
    amount: u32,
}

#[derive(Debug, Clone)]
struct Steel {
    amount: u32,
}

struct OreProcess {
    amount: u32,
}

impl Process for OreProcess {
    type RawMaterial = IronOre;
    type Product = Steel;

    fn push_along_the_belt(&self, material: Self::RawMaterial) -> Self::Product {
        // This is rather simple,
        // but we could of course make this much more complex if we wanted to
        Steel { amount: material.amount / 2 }
    }

    fn material(&self) -> Self::RawMaterial {
        IronOre { amount: self.amount }
    }
}

/// Do factory stuff.
fn main() {
    let ore_process = OreProcess { amount: 20 };
    let mut metal_factory: Factory<OreProcess> = Factory::new(0);
    metal_factory.add_new_process(ore_process);
    metal_factory.run();
}