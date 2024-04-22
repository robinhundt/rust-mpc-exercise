# MPC in Rust

This is a small project to get teach the basics of multi-party computation (MPC) and how it can be implemented in Rust. The goal of this project is to implement a simple version of the GMW protocol.

## Basics of Rust
First, you should familiarize yourself with the basics of Rust. A good starting point is the official [Learn Rust](https://www.rust-lang.org/learn) site. The [book](https://doc.rust-lang.org/book/) is a nice read which also teaches some important low-level concepts, the [rustlings](https://github.com/rust-lang/rustlings/) course is a nice practical introduction, where you learn by completing small automatically checked programming tasks. [cheats.rs](https://cheats.rs/) is an expansive cheat-sheet on Rust, perfect for looking up various things. Finally, I heavily recommend looking at the documentation of the [std library](https://doc.rust-lang.org/std/index.html), which contains lots of explanations and examples on how to use the various APIs. 

A note on the std library: Rust's std library is narrow but deep. E.g. look at the docs for the [Iterator](https://doc.rust-lang.org/std/iter/trait.Iterator.html) trait or the [Result](https://doc.rust-lang.org/std/result/enum.Result.html) type, which contain numerous convenience methods. However, a lot of functionality present in the standard library in other languages is provided by third-party crates (Rust jargon for libraries), which can be found via [crates.io](https://crates.io/) or [lib.rs](https://lib.rs/). One example of this is random number generation, for which the [rand](https://docs.rs/rand/latest/rand/) crate is the de facto standard choice.


### Clippy
Clippy is the official linter for Rust. Running `cargo clippy` will provide you with valuable feedback on your code.

### Editor
I heavily recommend to use either VSCode with the `rust-analyzer` plugin installed or the `RustRover` IDE by JetBrains. There is support for other editors by `rust-analyzer` (see [rust-analyzer.github.io](https://rust-analyzer.github.io/)). I would not recommend to program Rust without some form of language server, like `rust-analyzer` or the integrated `RustRover` IDE, as inline type-hints, suggestions, and inline docs severely help in learning Rust.

## Basics of MPC
A good introduction to MPC is the CRYPROT lecture by the ENCRYPTO group. Particularly the lecture on secure two-party computation, where the basic GMW protocol is explained, is relevant to this project.

Another great resource is the book [A Pragmatic Introduction to Secure Multi-Party Computation](https://securecomputation.org/main/). Chapter 3.2 and 3.4 are most relevant to this exercise, but I would heavily recommend to read the introduction (CH 1) and defining MPC (CH 2) chapters when you have time. 

## A simple GMW protocol implementation
The goal of this project is to program a simple implementation of the Goldreich-Micali-Wigderson (GMW) protocol ([[GMW87](https://dl.acm.org/doi/abs/10.1145/3335741.3335755)], however I would not spend too much time on the original paper, as the protocol is much more clear from the more recent material linked above).

In our implementation, we will first parse circuits in the [Bristol Fashion](https://nigelsmart.github.io/MPC-Circuits/) format and then execute these using Beaver's circuit randomization technique ([paper](https://link.springer.com/chapter/10.1007/3-540-46766-1_34) for completeness, but I recommend to look at the resources linked in Basics of MPC). In our implementation, we will limit ourselves to only two parties instead of n. These parties only communicate via an in-memory channel and not via the network.

### Where to start
The best place to start is the `src/circuit.rs` file, which already provides a basic structure for the parsing of the [Bristol circuits](https://nigelsmart.github.io/MPC-Circuits/). 


## Bonus Tasks
Following is a list of bonus tasks to tackle if you want to expand your implementation.
* Securely generate multiplication triples using oblivious transfer (OT).
  * As part of this, you can reimplement a base OT protocol like [[CO15](https://eprint.iacr.org/2015/267.pdf)] or use an existing OT library like our [ZappOT](https://github.com/encryptogroup/SEEC/tree/main/crates/zappot) library or the third-party [Ocelot](https://galoisinc.github.io/swanky/ocelot/) library.
* Perform actual communication via a TCP connection.
  * Note: Think about how you can minimize the number of communication rounds.
* Whatever you can think of... :)
