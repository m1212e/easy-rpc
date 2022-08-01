use std::path::Path;

use crate::transpiler::parser::{
    lexer::literal::LiteralType,
    parser::{
        custom_type::CustomType,
        endpoint::Endpoint,
        field_type::{ArrayAmount, PrimitiveType, Type},
    },
};

use self::{class::generate_class, interface::custom_type_to_interface};

use super::{Translator};

mod class;
mod endpoint;
mod interface;
mod tests;

pub struct TypeScriptTranslator;

impl Translator for TypeScriptTranslator {
    fn custom_type_to_interface(custom_type: &CustomType) -> String {
        custom_type_to_interface(custom_type)
    }

    fn generate_class(
        class_name: &str,
        relative_path: &str,
        endpoints: &Vec<Endpoint>,
        foreign: bool,
        class_imports: &Vec<String>,
        custom_types: &Vec<CustomType>,
    ) -> String {
        generate_class(
            class_name,
            relative_path,
            endpoints,
            foreign,
            class_imports,
            custom_types,
        )
    }

    fn file_suffix() -> String {
        String::from("ts")
    }
    
}

fn stringify_field_type(field_type: &Type) -> String {
    match field_type {
        Type::Primitive(primitive) => {
            let mut type_string = match primitive.primitive_type {
                PrimitiveType::Boolean => "boolean",
                PrimitiveType::Int8 => "number",
                PrimitiveType::Int16 => "number",
                PrimitiveType::Int32 => "number",
                PrimitiveType::Int64 => "number",
                PrimitiveType::Float32 => "number",
                PrimitiveType::Float64 => "number",
                PrimitiveType::String => "string",
            }
            .to_string();

            let array_string = match primitive.array_amount {
                ArrayAmount::NoArray => "",
                ArrayAmount::NoLengthSpecified => "[]",
                ArrayAmount::LengthSpecified(_) => "[]",
            };

            type_string.push_str(array_string);
            type_string
        }
        Type::Enum(en) => {
            let mut ret = String::new();
            for i in 0..en.values.len() {
                match &en.values[i].literal_type {
                    LiteralType::Boolean(val) => ret.push_str(&val.to_string()),
                    LiteralType::String(val) => {
                        ret.push_str("\"");
                        ret.push_str(&val);
                        ret.push_str("\"");
                    }
                    LiteralType::Float(val) => ret.push_str(&val.to_string()),
                    LiteralType::Integer(val) => ret.push_str(&val.to_string()),
                }

                if i < en.values.len() - 1 {
                    ret.push_str(" | ")
                }
            }

            ret
        }
        Type::Custom(custom) => match &custom.array_amount {
            ArrayAmount::NoArray => custom.identifier.to_string(),
            ArrayAmount::NoLengthSpecified => {
                let mut ret = String::new();
                ret.push_str(&custom.identifier);
                ret.push_str("[]");
                ret
            }
            ArrayAmount::LengthSpecified(_) => {
                let mut ret = String::new();
                ret.push_str(&custom.identifier);
                ret.push_str("[]");
                ret
            }
        },
    }
}
