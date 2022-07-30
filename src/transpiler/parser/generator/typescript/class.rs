use crate::transpiler::parser::parser::endpoint::Endpoint;

use super::{endpoint::endpoint_to_function, stringify_field_type};

pub fn generate_class(
    class_name: &str,
    relative_path: &str,
    endpoints: &Vec<Endpoint>,
    foreign: bool,
    imports: Option<Vec<&str>>,
) -> String {
    if foreign {
        generate_foreign_class(class_name, relative_path, endpoints)
    } else {
        generate_callback_class(class_name, relative_path, endpoints)
    }
}

fn generate_callback_class(
    class_name: &str,
    relative_path: &str,
    endpoints: &Vec<Endpoint>,
) -> String {
    let mut ret = String::new();

    ret.push_str("export default class ");
    ret.push_str(class_name);
    ret.push_str(" {
    private server: any
    /**
        This method is used by easy-rpc internally and is not intended for manual use. It can be used to set the server of the object.
    */
    private setERPCServer(server: any) {
        this.server = server

        // trigger the setters to set the handlers on the server object
");

    for endpoint in endpoints {
        ret.push_str("        this.");
        ret.push_str(&endpoint.identifier);
        ret.push_str(" = this.");
        ret.push_str(&endpoint.identifier);
        ret.push_str("\n");
    }

    ret.push_str("    }\n\n    constructor(callbacks?: {\n");

    for endpoint in endpoints {
        ret.push_str("        ");
        ret.push_str(&endpoint.identifier);
        ret.push_str(": (");

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

        ret.push_str(") => Promise<");
        if endpoint.return_type.is_some() {
            ret.push_str(&stringify_field_type(
                endpoint.return_type.as_ref().unwrap(),
            ));
        } else {
            ret.push_str("void");
        }
        ret.push_str(">\n");
    }
    ret.push_str("    }) {\n");

    for endpoint in endpoints {
        ret.push_str("        if (callbacks?.");
        ret.push_str(&endpoint.identifier);
        ret.push_str(") {\n            this.");
        ret.push_str(&endpoint.identifier);
        ret.push_str(" = callbacks.");
        ret.push_str(&endpoint.identifier);
        ret.push_str("\n        }\n\n");
    }
    ret.push_str("    }\n\n");

    for endpoint in endpoints {
        ret.push_str(&endpoint_to_function(
            endpoint,
            false,
            &format!("{}/{}/{}", relative_path, class_name, endpoint.identifier),
        ));
    }

    ret.push_str("}");

    ret
}

fn generate_foreign_class(
    class_name: &str,
    relative_path: &str,
    endpoints: &Vec<Endpoint>,
) -> String {
    let mut ret = String::new();

    ret.push_str("export default class ");
    ret.push_str(class_name);
    ret.push_str(" {\n    private server: any\n\n    constructor(server: any) {\n        this.server = server\n    }\n\n");

    for endpoint in endpoints {
        ret.push_str(&endpoint_to_function(endpoint, true, &format!("{}/{}/{}", relative_path, class_name, endpoint.identifier)))
    }

    ret.push_str("}");

    ret
}