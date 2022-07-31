use super::parser::{custom_type::CustomType, endpoint::Endpoint};
use std::collections::{hash_map::Entry, HashMap};

mod tests;
mod typescript;

/**
   Translates various easy-rpc elements into code strings of another language
*/
pub trait Translator {
    /**
       Translates a defined custom type into an interface of the target language
    */
    fn custom_type_to_interface(custom_type: &CustomType) -> String;

    /**
       Generates a class in the target language.

       The class name is the name used as classname.

       The relative path is used to build the endpoint identifier and should be the path of the class relative to the root folder. Path separators should be /.

       endpoints are all endpoints which the class should have.

       foreign indicates if the class should provide callbacks for handling incoming requests or methods for calling foreign endpoints.

       class_imports are all imports of sub classes which this class should import

       type_imports are all imports of custom types which are referenced in this class
    */
    fn generate_class(
        class_name: &str,
        relative_path: &str,
        endpoints: &Vec<Endpoint>,
        foreign: bool,
        class_imports: &Vec<Import>,
        type_imports: &Vec<Import>,
    ) -> String;
}

pub struct Import {
    source: String,
    name: String,
}

pub fn endpoints_to_classes_per_role<T: Translator>(
    class_name: &str,
    relative_path: &str,
    endpoints: Vec<Endpoint>,
    selected_role: &str,
    type_imports: &Vec<Import>,
    subclasses_to_import_per_role: HashMap<String, Vec<Import>>,
) -> HashMap<String, String> {
    let mut endpoints_per_role: HashMap<String, Vec<Endpoint>> = HashMap::new();

    for endpoint in endpoints {
        match endpoints_per_role.entry(endpoint.role.clone()) {
            Entry::Occupied(mut e) => {
                e.get_mut().push(endpoint);
            }
            Entry::Vacant(e) => {
                e.insert(vec![endpoint]);
            }
        }
    }

    let mut classes_by_role: HashMap<String, String> = HashMap::new();

    for (current_role, value) in endpoints_per_role.iter() {
        classes_by_role.insert(
            current_role.to_owned(),
            T::generate_class(
                class_name,
                relative_path,
                value,
                selected_role != current_role,
                subclasses_to_import_per_role
                    .get(current_role)
                    .unwrap_or(&vec![]),
                type_imports,
            ),
        );
    }
    classes_by_role
}
