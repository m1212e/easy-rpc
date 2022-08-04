use crate::transpiler::Role;

pub fn generate_client(
    foreign: bool,
    class_imports: &Vec<String>,
    role: Role,
    socket_enabled_browser_roles: &Vec<String>,
) -> String {
    if foreign {
        generate_foreign_client(
            class_imports,
            role,
            socket_enabled_browser_roles,
        )
    } else {
        generate_callback_client(
            class_imports,
            role,
            socket_enabled_browser_roles,
        )
    }
}

fn generate_callback_client(
    class_imports: &Vec<String>,
    role: Role,
    socket_enabled_browser_roles: &Vec<String>,
) -> String {
    let mut ret = String::new();

    // in case this is a browser, the length of the types vec will always be exactly 1
    let source = if role.types[0] == "browser" {
        "@easy-rpc/browser"
    } else {
        "@easy-rpc/node" // currently only supports node
    };
    ret.push_str(&format!(
        "import {{ ERPCServer, ServerOptions }} from \"{source}\"\n"
    ));

    for role in socket_enabled_browser_roles {
        ret.push_str(&format!("import {role} from \"./{role}\"\n"));
    }

    for imp in class_imports {
        ret.push_str(&format!(
            "import {imp} from \"./{rolename}/{imp}\"\n",
            rolename = role.name
        ));
    }
    ret.push_str("\n");

    match role.documentation {
        Some(doc) => {
            ret.push_str(&format!("/**{doc}*/"));
        }
        None => {}
    }

    ret.push_str(&format!(
        "export default class {class_name} extends ERPCServer {{\n",
        class_name=role.name
    ));

    for imp in class_imports {
        ret.push_str(&format!(
            "    private _{imp} = undefined as any
        set {imp}(value: {imp}) {{
            this._ {imp} = value
            (value as any).setERPCServer(this)
        }}
    get {imp}() {{
        return this._{imp}
    }}
"
        ));
    }

    ret.push_str(
        "    /**
        @param options The options to set for the easy-rpc object
        @param callbacks Callbacks to register for this server
    */
",
    );

    ret.push_str("    constructor(options: ServerOptions, callbacks: {");
    for imp in class_imports {
        ret.push_str(&format!("        {imp}: {imp}\n"))
    }
    ret.push_str("    }) {\n        super(options, [");
    for typ in role.types {
        ret.push_str(&format!("\"{typ}\", "));
    }

    let enable_websockets = socket_enabled_browser_roles.len() > 0;
    ret.push_str(&format!(
        "], {enable_websockets}, \"{role_name}\")\n",
        role_name = role.name
    ));

    for imp in class_imports {
        ret.push_str(&format!(
            "        if (callbacks.{imp}) {{
                this.{imp} = callbacks.{imp}
            }} else {{
            this.{imp} = new {imp}()
        }}
        "
        ));
    }

    if enable_websockets {
        ret.push_str("    onConnection(callback: (target: ");
        for i in 0..socket_enabled_browser_roles.len() {
            ret.push_str(&socket_enabled_browser_roles[i]);
            if i < socket_enabled_browser_roles.len() - 1 {
                ret.push_str(" | ");
            }
        }
        ret.push_str(
            ") => void) {\n        (super as any).onSocketConnection(({ role, client}) => {\n",
        );
        for role in socket_enabled_browser_roles {
            ret.push_str(&format!(
                "if (role == \"{role}\") {{
            const ret = new {role}()
            (ret as any).setERPCSocket(client)
            callback(ret)
        }}"
            ));
        }
        ret.push_str("    })\n}");
    }

    ret.push_str("}");

    ret
}

fn generate_foreign_client(
    class_imports: &Vec<String>,
    role: Role,
    socket_enabled_browser_roles: &Vec<String>,
) -> String {
    let mut ret = String::new();

    ret
}
