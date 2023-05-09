use crate::transpiler::config::Role;

pub fn generate_client(
    foreign: bool,
    class_imports: &Vec<String>,
    role: &Role,
    socket_enabled_browser_roles: &Vec<String>,
    library_source: &str,
) -> String {
    if foreign {
        generate_target(
            class_imports,
            role,
            socket_enabled_browser_roles,
            library_source,
        )
    } else {
        generate_server(
            class_imports,
            role,
            socket_enabled_browser_roles,
            library_source,
        )
    }
}

fn generate_server(
    class_imports: &Vec<String>,
    role: &Role,
    socket_enabled_browser_roles: &Vec<String>,
    library_source: &str,
) -> String {
    let mut ret = String::new();

    ret.push_str(&format!(
        "import {{ ERPCServer, ServerOptions }} from \"{library_source}\"\n"
    ));

    for browser_role in socket_enabled_browser_roles {
        if browser_role != &role.name {
            ret.push_str(&format!(
                "import {browser_role} from \"./{browser_role}\"\n"
            ));
        }
    }

    // websockets can be enabled in two cases:
    // 1: the client belongs to a server and there is at least one browser which has endpoints (socket_enabled_browser_roles.len() > 0)
    // 2: the client belongs to a browser and the role has endpoints (socket_enabled_browser_roles.contains(&role.name))
    let enable_websockets = (!socket_enabled_browser_roles.is_empty()
        && role.role_type.contains(&"http-server".to_string()))
        || (socket_enabled_browser_roles.contains(&role.name)
            && role.role_type.contains(&"browser".to_string()));

    for imp in class_imports {
        ret.push_str(&format!(
            "import {imp} from \"./{rolename}/{imp}\"\n",
            rolename = role.name
        ));
    }
    ret.push_str("\n");

    match &role.documentation {
        Some(doc) => {
            ret.push_str(&format!("/**{doc}*/\n"));
        }
        None => {}
    }

    ret.push_str(&format!(
        "export default class {class_name} extends ERPCServer {{\n",
        class_name = role.name
    ));

    for imp in class_imports {
        ret.push_str(&format!(
            "    private _{imp} = undefined as any
    set {imp}(value: {imp}) {{
        this._{imp} = value;
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

    ret.push_str("    constructor(options: ServerOptions, callbacks?: {\n");
    for imp in class_imports {
        ret.push_str(&format!("        {imp}: {imp}\n"))
    }
    ret.push_str(&format!("    }}) {{\n        super(options, \"{}\"", role.role_type));

    ret.push_str(&format!(
        ", {enable_websockets}, \"{role_name}\")\n",
        role_name = role.name
    ));

    for imp in class_imports {
        ret.push_str(&format!(
        "        if (callbacks?.{imp}) {{\n            this.{imp} = callbacks.{imp}\n        }} else {{\n            this.{imp} = new {imp}()\n        }}\n"
        ));
    }

    ret.push_str("    }\n");

    // browsers are not able to accept web socket connections, therefore we dont need to add the onConnection method to ws enabled browsers
    if enable_websockets && !role.role_type.contains(&"browser".to_string()) {
        ret.push_str("\n    onConnection(callback: (target: ");
        for i in 0..socket_enabled_browser_roles.len() {
            ret.push_str(&socket_enabled_browser_roles[i]);
            if i < socket_enabled_browser_roles.len() - 1 {
                ret.push_str(" | ");
            }
        }
        ret.push_str(
            ") => void) {
        // eslint-disable-next-line @typescript-eslint/ban-ts-comment
        // @ts-ignore
        super.onSocketConnection((role, socket) => {\n",
        );
        for role in socket_enabled_browser_roles {
            ret.push_str(&format!(
                "            if (role === \"{role}\") {{\n                const ret = new {role}({{address: \"\"}})\n                // eslint-disable-next-line @typescript-eslint/ban-ts-comment\n                // @ts-ignore\n                ret.setERPCSocket(socket)\n                callback(ret)\n            }}"
            ));
        }
        ret.push_str("\n        })\n    }");
    }

    ret.push_str("\n}");

    ret
}

fn generate_target(
    class_imports: &Vec<String>,
    role: &Role,
    socket_enabled_browser_roles: &Vec<String>,
    library_source: &str,
) -> String {
    let mut ret = String::new();

    ret.push_str(&format!(
        "import {{ ERPCTarget, TargetOptions }} from \"{library_source}\"\n"
    ));

    for browser_role in socket_enabled_browser_roles {
        if browser_role != &role.name {
            ret.push_str(&format!(
                "import {browser_role} from \"./{browser_role}\"\n"
            ));
        }
    }

    for imp in class_imports {
        ret.push_str(&format!(
            "import {imp} from \"./{rolename}/{imp}\"\n",
            rolename = role.name
        ));
    }
    ret.push_str("\n");

    match &role.documentation {
        Some(doc) => {
            ret.push_str(&format!("/**{doc}*/\n"));
        }
        None => {}
    }

    ret.push_str(&format!(
        "export default class {class_name} extends ERPCTarget {{\n",
        class_name = role.name
    ));

    for imp in class_imports {
        ret.push_str(&format!("    {imp} = new {imp}(this)\n"));
    }

    ret.push_str(
        "    /**
        @param options The options to set for the easy-rpc object
    */
",
    );

    ret.push_str("    constructor(options: TargetOptions) {\n");
    ret.push_str("        super(options, \"");
    ret.push_str(&role.role_type);

    ret.push_str("\")\n    }\n}");

    ret
}
