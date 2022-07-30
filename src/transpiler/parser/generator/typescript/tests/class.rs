#[cfg(test)]
mod tests {
    use crate::transpiler::parser::{
        generator::typescript::class::generate_class,
        parser::{
            endpoint::{Endpoint, Parameter},
            field_type::{ArrayAmount, Primitive, PrimitiveType, Type},
        },
        CodePosition,
    };

    #[test]
    fn test_success_callback() {
        let endpoints = vec![
            Endpoint {
                documentation: Some("some docs".to_string()),
                end: CodePosition::zero_initialized(),
                start: CodePosition::zero_initialized(),
                identifier: "MySuperCoolEndpoint1".to_string(),
                role: "MyVeryNiceRole".to_string(),
                return_type: Some(Type::Primitive(Primitive {
                    array_amount: ArrayAmount::NoLengthSpecified,
                    primitive_type: PrimitiveType::String,
                })),
                parameters: vec![
                    Parameter {
                        identifier: "p1".to_string(),
                        optional: true,
                        parameter_type: Type::Primitive(Primitive {
                            array_amount: ArrayAmount::NoLengthSpecified,
                            primitive_type: PrimitiveType::String,
                        }),
                    },
                    Parameter {
                        identifier: "p2".to_string(),
                        optional: false,
                        parameter_type: Type::Primitive(Primitive {
                            array_amount: ArrayAmount::NoArray,
                            primitive_type: PrimitiveType::Int8,
                        }),
                    },
                ],
            },
            Endpoint {
                documentation: None,
                end: CodePosition::zero_initialized(),
                start: CodePosition::zero_initialized(),
                identifier: "MySuperCoolEndpoint2".to_string(),
                role: "MyVeryNiceRole".to_string(),
                return_type: None,
                parameters: vec![],
            },
        ];

        let result = generate_class("MyCoolClass", "test/test2", &endpoints, false, None);

        assert_eq!(result, "export default class MyCoolClass {
    private server: any
    /**
        This method is used by easy-rpc internally and is not intended for manual use. It can be used to set the server of the object.
    */
    private setERPCServer(server: any) {
        this.server = server

        // trigger the setters to set the handlers on the server object
        this.MySuperCoolEndpoint1 = this.MySuperCoolEndpoint1
        this.MySuperCoolEndpoint2 = this.MySuperCoolEndpoint2
    }

    constructor(callbacks?: {
        MySuperCoolEndpoint1: (p1?: string[], p2: number) => Promise<string[]>
        MySuperCoolEndpoint2: () => Promise<void>
    }) {
        if (callbacks?.MySuperCoolEndpoint1) {
            this.MySuperCoolEndpoint1 = callbacks.MySuperCoolEndpoint1
        }

        if (callbacks?.MySuperCoolEndpoint2) {
            this.MySuperCoolEndpoint2 = callbacks.MySuperCoolEndpoint2
        }

    }

/**some docs*/
    private _MySuperCoolEndpoint1: (p1?: string[], p2: number) => Promise<string[]> = undefined as any
    set MySuperCoolEndpoint1(value: (p1?: string[], p2: number) => Promise<string[]>) {
        this._MySuperCoolEndpoint1 = value
        this.server?.registerERPCCallbackFunction(value, \"test/test2/MyCoolClass/MySuperCoolEndpoint1\")
    }
    get MySuperCoolEndpoint1() {
        return this._MySuperCoolEndpoint1
    }

    private _MySuperCoolEndpoint2: () => Promise<void> = undefined as any
    set MySuperCoolEndpoint2(value: () => Promise<void>) {
        this._MySuperCoolEndpoint2 = value
        this.server?.registerERPCCallbackFunction(value, \"test/test2/MyCoolClass/MySuperCoolEndpoint2\")
    }
    get MySuperCoolEndpoint2() {
        return this._MySuperCoolEndpoint2
    }

}");
    }

    #[test]
    fn test_success_foreign() {
        let endpoints = vec![
            Endpoint {
                documentation: Some("some docs".to_string()),
                end: CodePosition::zero_initialized(),
                start: CodePosition::zero_initialized(),
                identifier: "MySuperCoolEndpoint1".to_string(),
                role: "MyVeryNiceRole".to_string(),
                return_type: Some(Type::Primitive(Primitive {
                    array_amount: ArrayAmount::NoLengthSpecified,
                    primitive_type: PrimitiveType::String,
                })),
                parameters: vec![
                    Parameter {
                        identifier: "p1".to_string(),
                        optional: true,
                        parameter_type: Type::Primitive(Primitive {
                            array_amount: ArrayAmount::NoLengthSpecified,
                            primitive_type: PrimitiveType::String,
                        }),
                    },
                    Parameter {
                        identifier: "p2".to_string(),
                        optional: false,
                        parameter_type: Type::Primitive(Primitive {
                            array_amount: ArrayAmount::NoArray,
                            primitive_type: PrimitiveType::Int8,
                        }),
                    },
                ],
            },
            Endpoint {
                documentation: None,
                end: CodePosition::zero_initialized(),
                start: CodePosition::zero_initialized(),
                identifier: "MySuperCoolEndpoint2".to_string(),
                role: "MyVeryNiceRole".to_string(),
                return_type: None,
                parameters: vec![],
            },
        ];

        let result = generate_class("MyCoolClass", "test/test2", &endpoints, true, None);

        assert_eq!(result, "export default class MyCoolClass {
    private server: any

    constructor(server: any) {
        this.server = server
    }

/**some docs*/
    MySuperCoolEndpoint1(p1?: string[], p2: number): Promise<string[]> {
        return this.server.call(\"test/test2/MyCoolClass/MySuperCoolEndpoint1\", [p1, p2])
    }

    MySuperCoolEndpoint2(): Promise<void> {
        return this.server.call(\"test/test2/MyCoolClass/MySuperCoolEndpoint2\")
    }

}");
    }
}
