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
        Translates an endpoint to a function for the target language.
        The foreign parameter indicates if the generated code should be for calling and endpoint
        on another machine or to provide logic for handling a call on this machine.
        The url must be a unique identifier for determining this endpoint.
     */
    fn endpoint_to_function(endpoint: &Endpoint, foreign: bool, url: &str) -> String;
}
