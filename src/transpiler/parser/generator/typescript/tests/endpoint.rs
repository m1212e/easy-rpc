#[cfg(test)]
mod tests {
    use crate::transpiler::parser::{
        generator::{typescript::{endpoint::endpoint_to_function}},
        parser::{
            endpoint::{Endpoint, Parameter},
            field_type::{ArrayAmount, Primitive, PrimitiveType, Type},
        },
        CodePosition,
    };

    #[test]
    fn test_success_foreign() {
        let ep = Endpoint {
            documentation: Some("some docs".to_string()),
            end: CodePosition::zero_initialized(),
            start: CodePosition::zero_initialized(),
            identifier: "MySuperCoolEndpoint".to_string(),
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
        };

        let result = endpoint_to_function(&ep, true, "ABC123");

        assert_eq!(
            result,
            "/**some docs*/
    MySuperCoolEndpoint(p1?: string[], p2: number): Promise<string[]> {
        return this.server.call(\"ABC123\", [p1, p2])
    }

"
        )
    }

    #[test]
    fn test_success_callback() {
        let ep = Endpoint {
            documentation: Some("some docs".to_string()),
            end: CodePosition::zero_initialized(),
            start: CodePosition::zero_initialized(),
            identifier: "MySuperCoolEndpoint".to_string(),
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
        };

        let result = endpoint_to_function(&ep, false, "ABC123");

        assert_eq!(
            result,
            "/**some docs*/
    private _MySuperCoolEndpoint: (p1?: string[], p2: number) => Promise<string[]> = undefined as any
    set MySuperCoolEndpoint(value: (p1?: string[], p2: number) => Promise<string[]>) {
        this._MySuperCoolEndpoint = value
        this.server?.registerERPCCallbackFunction(value, \"ABC123\")
    }
    get MySuperCoolEndpoint() {
        return this._MySuperCoolEndpoint
    }

"
        )
    }
}

//TODO write some tests whith variation (no docs, no return type etc.)