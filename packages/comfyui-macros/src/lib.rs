use proc_macro::TokenStream;

mod macros;
mod options;

#[proc_macro_attribute]
pub fn node(arguments: TokenStream, input: TokenStream) -> TokenStream {
    macros::root::node(arguments, input)
}

#[proc_macro_derive(OutputPort)]
pub fn output_port_derive(input: TokenStream) -> TokenStream {
    macros::output::output_port_derive(input)
}

#[proc_macro_derive(InputDerive, attributes(attribute))]
pub fn input_derive(input: TokenStream) -> TokenStream {
    macros::input::input_derive(input)
}

#[proc_macro_derive(Enumerates)]
pub fn enumerates_derive(input: TokenStream) -> TokenStream {
    macros::r#enum::enumerates_derive(input)
}