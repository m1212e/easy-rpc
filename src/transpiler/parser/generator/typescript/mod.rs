use crate::transpiler::parser::{
    lexer::literal::LiteralType,
    parser::field_type::{ArrayAmount, PrimitiveType, Type},
};

mod interface;
mod tests;

pub struct TypeScriptTranslator;

fn stringify_field_type(field_type: Type) -> String {
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
            let mut values = en.values.iter().peekable();

            loop {
                let val = values.next();
                if val.is_none() {
                    break;
                }

                match &val.unwrap().literal_type {
                    LiteralType::Boolean(val) => ret.push_str(&val.to_string()),
                    LiteralType::String(val) => {
                        ret.push_str("\"");
                        ret.push_str(&val);
                        ret.push_str("\"");
                    }
                    LiteralType::Float(val) => ret.push_str(&val.to_string()),
                    LiteralType::Integer(val) => ret.push_str(&val.to_string()),
                }

                if values.peek().is_some() {
                    ret.push_str(" | ")
                }
            }

            ret
        }
        Type::Custom(mut custom) => match &custom.array_amount {
            ArrayAmount::NoArray => custom.identifier,
            ArrayAmount::NoLengthSpecified => {
                custom.identifier.push_str("[]");
                custom.identifier
            }
            ArrayAmount::LengthSpecified(_) => {
                custom.identifier.push_str("[]");
                custom.identifier
            }
        },
    }
}
