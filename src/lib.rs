
extern crate chrono;

#[cfg(test)]
#[macro_use]
extern crate matches;

/// Low level Teleinfo frame parsing.
pub mod frame;

/// Extracting information for the HC OptionTarifaire
pub mod hc;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
