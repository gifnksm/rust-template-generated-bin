//! Example project generated by gifnksm/rust-tmeplate
//!
//! # Usage
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! rust-template-generated-bin = "0.3.5"
//! ```

#![doc(html_root_url = "https://docs.rs/rust-template-generated-bin/0.3.5")]

use clap::Parser;

#[derive(Parser)]
pub struct Args {}

pub fn main() {
    let _args = Args::parse();
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
