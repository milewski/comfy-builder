// use std::collections::HashMap;
//
// pub struct NodeRegistry {
//     factories: HashMap<String, String>,
// }
//
// inventory::collect!(NodeRegistry);
//
// fn register_node<'a, T: Node<'a>>(
//     module: &'a Bound<'a, PyModule>,
//     class_mappings: &Bound<'a, PyDict>,
//     display_mappings: &Bound<'a, PyDict>,
// ) -> PyResult<()> {
//     module.add_class::<T>()?;
//
//     let type_name = std::any::type_name::<T>();
//     let class_name = type_name.split("::").last().unwrap_or(type_name);
//
//     class_mappings.set_item(class_name, python.get_type::<T>())?;
//     display_mappings.set_item(class_name, class_name)?;
//
//     Ok(())
// }