use crate::transpiler::parser::parser::endpoint::Endpoint;

use super::stringify_field_type;

/**
   Translates an endpoint to a function for the target language.
   The foreign parameter indicates if the generated code should be for calling and endpoint
   on another machine or to provide logic for handling a call on this machine.
   The url must be a unique identifier for determining this endpoint.
*/
pub fn endpoint_to_function(endpoint: &Endpoint, foreign: bool, url: &str) -> String {
    if foreign {
        make_foreign_endpoint(endpoint, url)
    } else {
        make_callback_endpoint(endpoint, url)
    }
}

fn make_foreign_endpoint(endpoint: &Endpoint, url: &str) -> String {
    let mut ret = String::new();

    match &endpoint.documentation {
        Some(val) => {
            ret.push_str(&format!("/**{val}*/\n"));
        }
        None => {}
    }

    ret.push_str(&format!(
        "    {identifier}(",
        identifier = &endpoint.identifier
    ));

    for i in 0..endpoint.parameters.len() {
        ret.push_str(&endpoint.parameters[i].identifier);

        if endpoint.parameters[i].optional {
            ret.push_str("?");
        }

        ret.push_str(": ");
        ret.push_str(&stringify_field_type(
            &endpoint.parameters[i].parameter_type,
        ));

        if i < endpoint.parameters.len() - 1 {
            ret.push_str(", ");
        }
    }

    ret.push_str("): Promise<");

    if endpoint.return_type.is_some() {
        ret.push_str(&stringify_field_type(
            endpoint.return_type.as_ref().unwrap(),
        ));
    } else {
        ret.push_str("void");
    }

    ret.push_str(&format!(
        "> {{
        return this.server.call(\"{url}\""
    ));

    if endpoint.parameters.len() > 0 {
        ret.push_str(", [");

        for i in 0..endpoint.parameters.len() {
            ret.push_str(&endpoint.parameters[i].identifier);
            if i < endpoint.parameters.len() - 1 {
                ret.push_str(", ");
            }
        }

        ret.push_str("]");
    }

    ret.push_str(")\n    }\n\n");

    ret
}

fn make_callback_endpoint(endpoint: &Endpoint, url: &str) -> String {
    let mut ret = String::new();

    if endpoint.documentation.is_some() {
        ret.push_str(&format!(
            "/**{}*/\n",
            endpoint.documentation.as_ref().unwrap()
        ));
    }

    ret.push_str(&format!("    private _{}: (", endpoint.identifier));

    let mut params_string = String::new();
    for i in 0..endpoint.parameters.len() {
        params_string.push_str(&endpoint.parameters[i].identifier);

        if endpoint.parameters[i].optional {
            params_string.push_str("?");
        }

        params_string.push_str(": ");
        params_string.push_str(&stringify_field_type(
            &endpoint.parameters[i].parameter_type,
        ));

        if i < endpoint.parameters.len() - 1 {
            params_string.push_str(", ");
        }
    }

    ret.push_str(&format!("{}) => Promise<", params_string));
    if endpoint.return_type.is_some() {
        ret.push_str(&stringify_field_type(
            endpoint.return_type.as_ref().unwrap(),
        ));
    } else {
        ret.push_str("void");
    }

    ret.push_str(&format!(
        "> = undefined as any\n    set {}(value: ({}) => Promise<",
        endpoint.identifier, params_string
    ));
    if endpoint.return_type.is_some() {
        ret.push_str(&stringify_field_type(
            endpoint.return_type.as_ref().unwrap(),
        ));
    } else {
        ret.push_str("void");
    }

    ret.push_str(&format!(
        ">) {{
        this._{id} = value
        this.server?.registerERPCHandler(value, \"{url}\")
    }}
    get {id}() {{
        return this._{id}
    }}

",
        id = endpoint.identifier
    ));

    ret
}
