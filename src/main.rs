use std::path::PathBuf;

pub mod circuit;
pub mod party;
pub mod mul_triple;

/// For argument parsing, my favorite crate is clap https://docs.rs/clap/latest/clap/
/// Especially its derive feature makes declarative argument parsing really easy.
/// You can add clap as a dependency with the derive feature and annotate this struct
/// and add the necessary fields.
struct Args {
    arg: PathBuf
}


fn main() {
    // The main function should first parse the passed arguments (I recommend to use a crate like
    // clap), and then evaluate the passed circuit. Note that you will likely need to run each
    // Party in its own thread (see https://doc.rust-lang.org/std/thread/index.html).
    println!("Hello, world!");
}
