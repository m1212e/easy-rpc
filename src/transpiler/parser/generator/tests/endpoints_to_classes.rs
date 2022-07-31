#[cfg(test)]
mod tests {
    use std::{collections::HashMap, vec};

    use crate::transpiler::parser::{
        generator::{endpoints_to_classes_per_role, typescript::TypeScriptTranslator, Import},
        parser::endpoint::Endpoint,
        CodePosition,
    };

    #[test]
    fn test_success() {
        let result = endpoints_to_classes_per_role::<TypeScriptTranslator>(
            "TestClass",
            "test/test2",
            vec![
                Endpoint {
                    documentation: None,
                    end: CodePosition::zero_initialized(),
                    start: CodePosition::zero_initialized(),
                    identifier: "MySuperCoolEndpoint".to_string(),
                    role: "Server".to_string(),
                    return_type: None,
                    parameters: vec![],
                },
                Endpoint {
                    documentation: None,
                    end: CodePosition::zero_initialized(),
                    start: CodePosition::zero_initialized(),
                    identifier: "MySuperCoolEndpoint2".to_string(),
                    role: "Client".to_string(),
                    return_type: None,
                    parameters: vec![],
                },
            ],
            "Server",
            &vec![],
            HashMap::from([
                (
                    "Server".to_string(),
                    vec![Import {
                        name: "someName".to_string(),
                        source: "someSource".to_string(),
                    }],
                ),
                (
                    "Client".to_string(),
                    vec![Import {
                        name: "someName2".to_string(),
                        source: "someSource2".to_string(),
                    }],
                ),
            ]),
        );

        assert_eq!(result.get("Server").unwrap(), "import someName from \"./someSource/someName\"

export default class TestClass {
    private server: any
    /**
        This method is used by easy-rpc internally and is not intended for manual use. It can be used to set the server of the object.
    */
    private setERPCServer(server: any) {
        this.server = server

        // trigger the setters to set the handlers on the server object
        this.MySuperCoolEndpoint = this.MySuperCoolEndpoint
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
        this._someName = value
        (value as any).setERPCServer(this.server)
    }
    get someName() {
        return this._someName
    }

}");

        assert_eq!(result.get("Client").unwrap(), "import someName2 from \"./someSource2/someName2\"

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

}");
    }
}
