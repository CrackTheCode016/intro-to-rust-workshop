# WebZero Hackerhouse Rust Workshop - An Intro to Rust

> The slides are available on hackMD, but also here in the repo:[ HACKMD.md.](HACKMD.md)

This workshop focuses on two main objectives, mainly: 

1. Providing a basic introduction to Rust, and the necessary features to complete the workshop
2. Complete the "story" using those concepts

Note that some of these concepts are not necessarily blockchain specific, but analogies can easily be made, making this a versatile workshop.

This workshop was created to take around **1 hour**, as it has them potentially install Rust, then go over its basics and how they roughly work.

It is also catered to a Substrate-style version of development, in which types are used as the bounds for configuring various traits.  The goal is to expose the students to a generic-style API, although this isn't compressive, it serves as an intro to some of these concepts.

## Context: The Factory

For this workshop, we will go launch a **Factory**.  This factory has the following flow:

- We want to process a raw material to a product (`Product`).
- The processes, in which our products go on to be processed
- Each `Process` also has a configuration, in which we utilize traits
- Lastly, once the `Product` passes its specific belt process, it is saved as a completed item in our "storage" (a `HashMap`, in our case).


### Analogies to draw

- The factory is really a sort of state machine, accepting inputs, transforming them somehow, then storing them again. 
- A process is analogous to a state transition.  It is also kind of a glorified conversion.
- The factory, could in theory, receive products from other factories to further process.

## An example scenario: IronOre -> Steel

In our scenario, we have a raw material, iron ore (`IronOre`), and we wish to process it into steel (`Steel`). 

- We must declare a new Product, "Steel"
- Our belt accepts iron ore, and outputs steel for `push_along_the_belt`
- Steel gets deposited into a `HashMap` as a "complete product".

## Example steps to follow

See the hackMD slide readme for more detail:[ HACKMD.](./HACKMD.md)

