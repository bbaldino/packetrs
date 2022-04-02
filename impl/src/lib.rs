//! This crate implements the macro for `packetrs` and should not be used directly.

mod code_gen;
pub mod error;
mod model_parse;
mod model_types;
pub mod packetrs_read;
mod syn_helpers;

use proc_macro2::TokenStream;
use syn::DeriveInput;

//pub use ::anyhow::{Context, Error, Result};
pub use ::anyhow::*;
pub use ::bitcursor::*;

use crate::{code_gen::generate_struct, model_parse::parse_struct};

#[doc(hidden)]
pub fn derive_packetrs(item: TokenStream) -> std::result::Result<TokenStream, syn::Error> {
    //pub fn derive_packetrs(item: TokenStream) -> std::result::Result<TokenStream, syn::Error> {
    let ast: DeriveInput = syn::parse2(item)?;
    //println!("got ast: {:#?}", ast);
    match ast.data {
        syn::Data::Struct(ref s) => {
            let parsed = parse_struct(&ast.ident, &ast.attrs, s);
            //eprintln!("Parsed struct: {:#?}", parsed);
            Ok(generate_struct(&parsed)).map_err(|e: anyhow::Error| syn::Error::new_spanned(ast, e))
        }
        syn::Data::Enum(ref e) => {
            //let parsed = parse_enum(&ast.ident, &ast.attrs, e);
            //eprintln!("Parsed enum: {:#?}", parsed);
            //derive_enum(&parsed)
            Ok(TokenStream::new()).map_err(|e: anyhow::Error| syn::Error::new_spanned(ast, e))
        }
        _ => Err(syn::Error::new_spanned(
            ast,
            "Packetrs is only supported on structs and enums",
        )),
    }
}
