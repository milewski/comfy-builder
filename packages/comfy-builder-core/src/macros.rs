#[macro_export]
macro_rules! set_defaults {
    ($dict:expr, $( $key:expr => $value:expr ),* $(,)?) => {
        $(
            if let Err(_) = $dict.get_item($key) {
                $dict.set_item($key, $value)?;
            }
        )*
    };
}

#[macro_export]
macro_rules! run_node {
    ($node:ident, $input:expr) => {{
        match $node::new().execute($input) {
            Ok(output) => output,
            Err(error) => panic!("`{}` node failed: {}", stringify!($node), error),
        }
    }};
}