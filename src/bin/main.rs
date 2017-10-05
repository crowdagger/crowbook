extern crate crowbook;
extern crate crowbook_intl_runtime;
extern crate indicatif;
#[macro_use]
extern crate log;

#[cfg(feature= "binary")]
extern crate simplelog;

#[cfg(feature = "binary")]
extern crate clap;

#[macro_use]
mod localize_macros;
#[cfg(feature = "binary")]
mod real_main;
#[cfg(feature = "binary")]
mod helpers;
#[cfg(feature = "binary")]



#[cfg(feature = "binary")]
#[macro_use]
extern crate lazy_static;

#[cfg(feature = "binary")]
fn main() {
    real_main::real_main();
}

#[cfg(not(feature = "binary"))]
fn main() {
    println!("In order to work, the binary must be compiled with the \"binary\" feature.");
}
