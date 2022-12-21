#[cfg(test)]
mod tests {
    use tower_lsp::lsp_types::Range;

    use crate::transpiler::{
        generator::translator::typescript::endpoint::endpoint_to_function,
        parser::parser::{
            endpoint::{Endpoint, Parameter},
            erpc_type::{ArrayAmount, Primitive, PrimitiveType, Type},
        },
    };

    #[test]
    fn test_success_foreign() {
        let ep = Endpoint {
            middleware_identifiers: vec![],
            documentation: Some("some docs".to_string()),
            range: Range::default(),
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
            middleware_identifiers: vec![],
            documentation: Some("some docs".to_string()),
            range: Range::default(),
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
        this.server?.registerERPCHandler(value, \"ABC123\")
    }
    get MySuperCoolEndpoint() {
        return this._MySuperCoolEndpoint
    }

"
        )
    }
}

//TODO write some tests whith variation (no docs, no return type etc.)
