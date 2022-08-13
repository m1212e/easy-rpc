use crate::transpiler::Role;

pub fn generate_client(
    foreign: bool,
    class_imports: &Vec<String>,
    role: &Role,
    socket_enabled_browser_roles: &Vec<String>,
    library_source: &str
) -> String {
    if foreign {
        generate_foreign_client(class_imports, role, socket_enabled_browser_roles, library_source)
    } else {
        generate_callback_client(class_imports, role, socket_enabled_browser_roles, library_source)
    }
}

fn generate_callback_client(
    class_imports: &Vec<String>,
    role: &Role,
    socket_enabled_browser_roles: &Vec<String>,
    library_source: &str
) -> String {
    let mut ret = String::new();

    ret.push_str(&format!(
        "import {{ ERPCServer, ServerOptions }} from \"{library_source}\"\n"
    ));

    for browser_role in socket_enabled_browser_roles {
        if browser_role != &role.name {
            ret.push_str(&format!("import {browser_role} from \"./{browser_role}\"\n"));
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

    ret.push_str("    constructor(options: ServerOptions, callbacks: {\n");
    for imp in class_imports {
        ret.push_str(&format!("        {imp}: {imp}\n"))
    }
    ret.push_str("    }) {\n        super(options, [");
    for typ in &role.types {
        ret.push_str(&format!("\"{typ}\", "));
    }

    let enable_websockets = socket_enabled_browser_roles.len() > 0;
    ret.push_str(&format!(
        "], {enable_websockets}, \"{role_name}\")\n",
        role_name = role.name
    ));

    for imp in class_imports {
        ret.push_str(&format!(
        "        if (callbacks.{imp}) {{\n            this.{imp} = callbacks.{imp}\n        }} else {{\n            this.{imp} = new {imp}()\n        }}\n"
        ));
    }

    ret.push_str("    }\n");

    if enable_websockets {
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
        super.onSocketConnection(({ role, client}) => {\n",
        );
        for role in socket_enabled_browser_roles {
            ret.push_str(&format!(
                "            if (role == \"{role}\") {{\n                const ret = new {role}({{}} as any)\n                // eslint-disable-next-line @typescript-eslint/ban-ts-comment\n                // @ts-ignore\n                ret.setERPCSocket(client)\n                callback(ret)\n            }}"
            ));
        }
        ret.push_str("\n        })\n    }");
    }

    ret.push_str("\n}");

    ret
}

fn generate_foreign_client(
    class_imports: &Vec<String>,
    role: &Role,
    socket_enabled_browser_roles: &Vec<String>,
    library_source: &str
) -> String {
    let mut ret = String::new();

    ret.push_str(&format!(
        "import {{ ERPCTarget, TargetOptions }} from \"{library_source}\"\n"
    ));

    for browser_role in socket_enabled_browser_roles {
        if browser_role != &role.name {
            ret.push_str(&format!("import {browser_role} from \"./{browser_role}\"\n"));
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
    ret.push_str("        super(options, [");
    for typ in &role.types {
        ret.push_str(&format!("\"{typ}\", "));
    }

    ret.push_str("])\n    }\n}");

    ret
}
