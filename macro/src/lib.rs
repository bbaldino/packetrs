//! This crate implements the macro for `packetrs` and should not be used directly.

extern crate proc_macro;

use proc_macro::TokenStream;
use syn::parse_macro_input;

#[proc_macro_derive(PacketrsRead, attributes(packetrs))]
/// Document your macro here.
pub fn derive_packetrs_read(item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as proc_macro2::TokenStream);

    match packetrs_impl::derive_packetrs_read(item) {
        Ok(tokens) => tokens.into(),
        Err(err) => TokenStream::from(err.to_compile_error()),
    }
}
