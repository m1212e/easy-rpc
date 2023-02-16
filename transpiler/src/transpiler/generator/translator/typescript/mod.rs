use crate::transpiler::{
    config::Role,
    parser::{
        lexer::literal::LiteralType,
        parser::{
            custom_type::CustomType,
            endpoint::Endpoint,
            erpc_type::{ArrayAmount, EnumType, Primitive, PrimitiveType, Type},
        },
    },
};

use self::{class::generate_class, client::generate_client};

use super::Translator;

mod class;
mod client;
mod endpoint;
mod interface;
mod tests;

pub struct TypeScriptTranslator;

impl Translator for TypeScriptTranslator {
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

    fn generate_client(
        foreign: bool,
        class_imports: &Vec<String>,
        role: &Role,
        socket_enabled_browser_roles: &Vec<String>,
        library_source: &str,
    ) -> String {
        generate_client(
            foreign,
            class_imports,
            role,
            socket_enabled_browser_roles,
            library_source,
        )
    }
}

fn stringify_field_type(field_type: &Type) -> String {
    match field_type {
        Type::Primitive(primitive) => stringify_primitive(primitive),
        Type::Enum(en) => {
            let mut ret = String::new();
            for i in 0..en.values.len() {
                match &en.values[i] {
                    EnumType::Primitive(primitive) => ret.push_str(&stringify_primitive(primitive)),
                    EnumType::Custom(custom) => {
                        ret.push_str(&custom.identifier);
                        match custom.array_amount {
                            ArrayAmount::NoArray => {}
                            ArrayAmount::NoLengthSpecified => ret.push_str("[]"),
                            ArrayAmount::LengthSpecified(_) => ret.push_str("[]"),
                        };
                    }
                    EnumType::Literal(literal) => match literal {
                        LiteralType::Boolean(val) => ret.push_str(&val.to_string()),
                        LiteralType::String(val) => {
                            ret.push_str(&format!("\"{val}\""));
                        }
                        LiteralType::Float(val) => ret.push_str(&val.to_string()),
                        LiteralType::Integer(val) => ret.push_str(&val.to_string()),
                    },
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
                ret.push_str(&format!("{}[]", custom.identifier));
                ret
            }
            ArrayAmount::LengthSpecified(_) => {
                let mut ret = String::new();
                ret.push_str(&format!("{}[]", custom.identifier));
                ret
            }
        },
    }
}

fn stringify_primitive(primitive: &Primitive) -> String {
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
