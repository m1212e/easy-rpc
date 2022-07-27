use super::parser::custom_type::CustomType;

mod typescript;

/**
    Translates various easy-rpc elements into code strings of another language
 */
pub trait Translator {
    /**
        Translates a defined custom type into an interface of the target language
     */
    fn custom_type_to_interface(custom_type: CustomType) -> String;
}
