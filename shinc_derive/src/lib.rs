extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod derives;

#[proc_macro_derive(CliCommand)]
pub fn cli_command(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    derives::derive_cli_command(&input).unwrap_or_else(|err| err.to_compile_error()).into()
}
