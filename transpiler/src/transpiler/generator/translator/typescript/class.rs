use crate::transpiler::parser::parser::{custom_type::CustomType, endpoint::Endpoint};

use super::{
    endpoint::endpoint_to_function, interface::custom_type_to_interface, stringify_field_type,
};

pub fn generate_class(
    class_name: &str,
    relative_path: &str,
    endpoints: &Vec<Endpoint>,
    foreign: bool,
    class_imports: &Vec<String>,
    custom_types: &Vec<CustomType>,
) -> String {
    if foreign {
        generate_foreign_class(
            class_name,
            relative_path,
            endpoints,
            class_imports,
            custom_types,
        )
    } else {
        generate_callback_class(
            class_name,
            relative_path,
            endpoints,
            class_imports,
            custom_types,
        )
    }
}

fn generate_callback_class(
    class_name: &str,
    relative_path: &str,
    endpoints: &Vec<Endpoint>,
    class_imports: &Vec<String>,
    custom_types: &Vec<CustomType>,
) -> String {
    let mut ret = String::new();

    for imp in class_imports {
        ret.push_str(&format!("import {imp} from \"./{class_name}/{imp}\"\n"));
    }
    ret.push_str("\n");

    for t in custom_types {
        ret.push_str(&custom_type_to_interface(t));
        ret.push_str("\n");
    }

    ret.push_str(&format!("export default class {class_name} {{
    private server: any
    /**
        This method is used by easy-rpc internally and is not intended for manual use. It can be used to set the server of the object.
    */
    private setERPCServer(server: any) {{
        this.server = server

        // trigger the setters to set the handlers on the server object
"));

    for endpoint in endpoints {
        let identifier = &endpoint.identifier;
        ret.push_str(&format!(
            "        if (this.{identifier}) {{
            this.{identifier} = this.{identifier}
        }}
"
        ));
    }

    ret.push_str("    }\n\n    constructor(callbacks?: {\n");

    for endpoint in endpoints {
        ret.push_str(&format!("        {}: (", endpoint.identifier));

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

    for imp in class_imports {
        ret.push_str(&format!("        {imp}: {imp}\n"));
    }

    ret.push_str("    }) {\n");

    for endpoint in endpoints {
        ret.push_str(&format!("        if (callbacks?.{id}) {{
            this.{id} = callbacks.{id}
        }}

", id=endpoint.identifier));
    }

    for imp in class_imports {
        ret.push_str(&format!("        if (callbacks?.{imp}) {{
            this.{imp} = callbacks.{imp}
        }} else {{
            this.{imp} = this.{imp}
        }}

"));
    }

    ret.push_str("    }\n\n");

    for endpoint in endpoints {
        ret.push_str(&endpoint_to_function(
            endpoint,
            false,
            &format!("{relative_path}{class_name}/{}", endpoint.identifier),
        ));
    }

    for imp in class_imports {
        ret.push_str(&format!(
            "    private _{imp} = new {imp}()
    set {imp}(value: {imp}) {{
        this._{imp} = value;
        (value as any).setERPCServer(this.server)
    }}
    get {imp}() {{
        return this._{imp}
    }}
"
        ));
    }

    ret.push_str("\n}");

    ret
}

fn generate_foreign_class(
    class_name: &str,
    relative_path: &str,
    endpoints: &Vec<Endpoint>,
    class_imports: &Vec<String>,
    custom_types: &Vec<CustomType>,
) -> String {
    let mut ret = String::new();

    for imp in class_imports {
        
        ret.push_str(&format!("import {imp} from \"./{class_name}/{imp}\"\n"));
    }
    ret.push_str("\n");

    for t in custom_types {
        ret.push_str(&custom_type_to_interface(t));
        ret.push_str("\n");
    }

    ret.push_str(&format!("export default class {class_name} {{\n"));

    for imp in class_imports {
        ret.push_str(&format!("    {imp}: {imp}\n"));
    }
    ret.push_str("\n");

    ret.push_str(
        "    private server: any\n\n    constructor(server: any) {\n        this.server = server\n",
    );

    for imp in class_imports {
        ret.push_str(&format!("        this.{imp} = new {imp}(server)\n"));
    }

    ret.push_str("    }\n\n");

    for endpoint in endpoints {
        ret.push_str(&endpoint_to_function(
            endpoint,
            true,
            &format!("{}{}/{}", relative_path, class_name, endpoint.identifier),
        ))
    }

    ret.push_str("}");

    ret
}
