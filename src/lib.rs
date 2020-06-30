// to avoid the warning from diesel macros
#![allow(proc_macro_derive_resolution_fallback)]
#![allow(warnings)]
#![recursion_limit = "128"]

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate log;
#[macro_use]
extern crate json;

pub mod utils;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
