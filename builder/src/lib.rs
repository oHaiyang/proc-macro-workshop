use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};


#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let _ = parse_macro_input!(input as DeriveInput);
    
    TokenStream::new()
}
