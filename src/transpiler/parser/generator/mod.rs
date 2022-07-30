use super::parser::{custom_type::CustomType, endpoint::Endpoint};

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
