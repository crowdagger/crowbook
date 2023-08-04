extern crate crowbook;
extern crate crowbook_intl_runtime;
extern crate yaml_rust;

#[macro_use]
mod localize_macros;
#[cfg(feature = "binary")]
mod helpers;
#[cfg(feature = "binary")]
mod real_main;
#[cfg(feature = "binary")]
#[cfg(feature = "binary")]
#[macro_use]
extern crate lazy_static;

#[cfg(feature = "binary")]
fn main() {
    crate::real_main::real_main();
}

#[cfg(not(feature = "binary"))]
fn main() {
    println!("In order to work, the binary must be compiled with the \"binary\" feature.");
}
