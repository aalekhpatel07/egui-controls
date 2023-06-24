mod ui;
mod parse;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};


#[proc_macro_derive(ControlPanel, attributes(control))]
pub fn derive(input: TokenStream) -> TokenStream {
    parse::expand(parse_macro_input!(input as DeriveInput))
}
