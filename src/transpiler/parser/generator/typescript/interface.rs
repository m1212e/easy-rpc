use crate::transpiler::parser::parser::custom_type::CustomType;

use super::{stringify_field_type};

pub fn custom_type_to_interface(custom_type: &CustomType) -> String {
    let mut builder = String::new();

    if custom_type.documentation.is_some() {
        builder.push_str("/**");
        builder.push_str(&custom_type.documentation.as_ref().unwrap());
        builder.push_str("*/\n");
    }

    builder.push_str("interface ");
    builder.push_str(&custom_type.identifier);
    builder.push_str(" {\n");

    for field in &custom_type.fields {
        if field.documentation.is_some() {
            builder.push_str("/**");
            builder.push_str(&field.documentation.as_ref().unwrap());
            builder.push_str("*/\n");
        }

        builder.push_str("    ");
        builder.push_str(&field.identifier);

        if field.optional {
            builder.push_str("?");
        }

        builder.push_str(": ");
        builder.push_str(&stringify_field_type(&field.field_type));
        builder.push_str("\n");
    }

    builder.push_str("}\n");
    return builder;
}
