#[cfg(test)]
mod tests {
    use std::{collections::HashMap, vec};

    use tower_lsp::lsp_types::Range;

    use crate::transpiler::{
        generator::{generate_classes_per_role, translator::typescript::TypeScriptTranslator},
        parser::{parser::endpoint::Endpoint},
    };

    #[test]
    fn test_success() {
        let result = generate_classes_per_role::<TypeScriptTranslator>(
            "TestClass",
            "test/test2/",
            vec![
                Endpoint {
                    documentation: None,
                    range: Range::default(),
                    identifier: "MySuperCoolEndpoint".to_string(),
                    role: "Server".to_string(),
                    return_type: None,
                    parameters: vec![],
                },
                Endpoint {
                    documentation: None,
                    range: Range::default(),
                    identifier: "MySuperCoolEndpoint2".to_string(),
                    role: "Client".to_string(),
                    return_type: None,
                    parameters: vec![],
                },
                Endpoint {
                    documentation: None,
                    range: Range::default(),
                    identifier: "MySuperCoolEndpoint3".to_string(),
                    role: "Client".to_string(),
                    return_type: None,
                    parameters: vec![],
                },
            ],
            "Server",
            &vec![],
            &HashMap::from([
                (
                    "Server".to_string(),
                    vec!["someName".to_string(),],
                ),
                (
                    "Client".to_string(),
                    vec!["someName2".to_string(),],
                ),
            ]),
        );

        assert_eq!(result.get("Server").unwrap(), "import someName from \"./TestClass/someName\"

export default class TestClass {
    private server: any
    /**
        This method is used by easy-rpc internally and is not intended for manual use. It can be used to set the server of the object.
    */
    private setERPCServer(server: any) {
        this.server = server

        // trigger the setters to set the handlers on the server object
        if (this.MySuperCoolEndpoint) {
            this.MySuperCoolEndpoint = this.MySuperCoolEndpoint
        }
    }

    constructor(callbacks?: {
        MySuperCoolEndpoint: () => Promise<void>
        someName: someName
    }) {
        if (callbacks?.MySuperCoolEndpoint) {
            this.MySuperCoolEndpoint = callbacks.MySuperCoolEndpoint
        }

        if (callbacks?.someName) {
            this.someName = callbacks.someName
        } else {
            this.someName = this.someName
        }

    }

    private _MySuperCoolEndpoint: () => Promise<void> = undefined as any
    set MySuperCoolEndpoint(value: () => Promise<void>) {
        this._MySuperCoolEndpoint = value
        this.server?.registerERPCCallbackFunction(value, \"test/test2/TestClass/MySuperCoolEndpoint\")
    }
    get MySuperCoolEndpoint() {
        return this._MySuperCoolEndpoint
    }

    private _someName = new someName()
    set someName(value: someName) {
        this._someName = value;
        (value as any).setERPCServer(this.server)
    }
    get someName() {
        return this._someName
    }

}");

        assert_eq!(
            result.get("Client").unwrap(),
            "import someName2 from \"./TestClass/someName2\"

export default class TestClass {
    someName2: someName2

    private server: any

    constructor(server: any) {
        this.server = server
        this.someName2 = new someName2(server)
    }

    MySuperCoolEndpoint2(): Promise<void> {
        return this.server.call(\"test/test2/TestClass/MySuperCoolEndpoint2\")
    }

    MySuperCoolEndpoint3(): Promise<void> {
        return this.server.call(\"test/test2/TestClass/MySuperCoolEndpoint3\")
    }

}"
        );
    }
}
