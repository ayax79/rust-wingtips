#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

pub mod span;
pub mod errors;
mod helpers;

//mod tracer;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
