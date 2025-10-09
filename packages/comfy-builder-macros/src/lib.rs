use proc_macro::TokenStream;

mod macros;
mod options;
mod helpers;

#[proc_macro_attribute]
pub fn node(arguments: TokenStream, input: TokenStream) -> TokenStream {
    macros::node::node(arguments, input)
}

#[proc_macro_attribute]
pub fn node_v3(arguments: TokenStream, input: TokenStream) -> TokenStream {
    macros::node_v3::node(arguments, input)
}

#[proc_macro_derive(NodeOutput, attributes(label, tooltip))]
pub fn node_output_derive(input: TokenStream) -> TokenStream {
    macros::output::node_output_derive(input)
}

#[proc_macro_derive(NodeInput, attributes(attribute, label))]
pub fn node_input_derive(input: TokenStream) -> TokenStream {
    macros::input::node_input_derive(input)
}

#[proc_macro_derive(Enum, attributes(label))]
pub fn enum_derive(input: TokenStream) -> TokenStream {
    macros::r#enum::enum_derive(input)
}

#[proc_macro]
pub fn boostrap(input: TokenStream) -> TokenStream {
    macros::boostrap::boostrap(input)
}