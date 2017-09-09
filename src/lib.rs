
extern crate chrono;

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
