use proc_macro::TokenStream;

#[proc_macro_derive(ExperimentalApi, attributes(experimental))]
pub fn derive_experimental_api(_input: TokenStream) -> TokenStream {
    TokenStream::new()
}
