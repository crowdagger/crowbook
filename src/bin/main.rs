extern crate crowbook;

#[cfg(feature = "clap")]
extern crate clap;
#[cfg(feature = "clap")]
mod real_main;
#[cfg(feature = "clap")]
mod helpers;

#[cfg(feature = "clap")]
fn main() {
    real_main::real_main();
}

#[cfg(not(feature = "clap"))]
fn main() {
    println!("Clap dependency is required to build the binary.");
}
    
