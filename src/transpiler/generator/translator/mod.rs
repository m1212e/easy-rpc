use crate::transpiler::{
    parser::parser::{custom_type::CustomType, endpoint::Endpoint}, config::Role,
};

pub mod typescript;

/**
   Translates various easy-rpc elements into code strings of another language
*/
pub trait Translator {
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
        class_imports: &Vec<String>,
        custom_types: &Vec<CustomType>,
    ) -> String;

    //TODO: make the file suffix return a slice instead of a string object
    /**
       Returns the file suffix for the generated language.

       E.g. TypeScript -> ts
       E.g. Rust -> rs
    */
    fn file_suffix() -> String;

    /**
       Generate the client class actually used by the user
    */
    fn generate_client(
        foreign: bool,
        class_imports: &Vec<String>,
        role: &Role,
        socket_enabled_browser_roles: &Vec<String>,
        library_source: &str,
    ) -> String;
}
