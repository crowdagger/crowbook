#[macro_use]
#[cfg(feature = "binary")]
mod helpers;
#[cfg(feature = "binary")]
mod real_main;
#[cfg(feature = "binary")]
#[cfg(feature = "binary")]
#[macro_use]
extern crate lazy_static;

rust_i18n::i18n!("lang/bin", fallback="en");


#[cfg(feature = "binary")]
fn main() {
    crate::real_main::real_main();
}

#[cfg(not(feature = "binary"))]
fn main() {
    println!("In order to work, the binary must be compiled with the \"binary\" feature.");
}
