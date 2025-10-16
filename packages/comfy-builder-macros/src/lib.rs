use proc_macro::TokenStream;

mod helpers;
mod macros;

#[proc_macro_attribute]
pub fn node(arguments: TokenStream, input: TokenStream) -> TokenStream {
    macros::node::node(arguments, input)
}

#[proc_macro_derive(NodeOutput, attributes(label, display_name, tooltip))]
pub fn node_output_derive(input: TokenStream) -> TokenStream {
    macros::output::node_output_derive(input)
}

#[proc_macro_derive(
    NodeInput,
    attributes(
        attribute,
        default,
        display_name,
        display_mode,
        tooltip,
        placeholder,
        min,
        max,
        label_on,
        label_off,
        multiline,
        control_after_generate
    )
)]
pub fn node_input_derive(input: TokenStream) -> TokenStream {
    macros::input::node_input_derive(input)
}

#[proc_macro_derive(Enum, attributes(display_name))]
pub fn enum_derive(input: TokenStream) -> TokenStream {
    macros::r#enum::enum_derive(input)
}

#[proc_macro]
pub fn boostrap(input: TokenStream) -> TokenStream {
    macros::boostrap::boostrap(input)
}
