#[cfg(test)]
mod tests {
    use crate::transpiler::{
        parser::{
            parser::{
                custom_type::CustomType,
                endpoint::{Endpoint, Parameter},
                field_type::{ArrayAmount, Primitive, PrimitiveType, Type},
            },
            CodePosition,
        }, generator::translator::typescript::class::generate_class,
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

        let result = generate_class(
            "MyCoolClass",
            "test/test2/",
            &endpoints,
            false,
            &vec!["ImportedClass".to_string(), "ImportedClass2".to_string()],
            &vec![
                CustomType {
                    documentation: None,
                    start: CodePosition::zero_initialized(),
                    end: CodePosition::zero_initialized(),
                    fields: vec![],
                    identifier: "MyCoolType1".to_string(),
                },
                CustomType {
                    documentation: None,
                    start: CodePosition::zero_initialized(),
                    end: CodePosition::zero_initialized(),
                    fields: vec![],
                    identifier: "MyCoolType2".to_string(),
                },
            ],
        );

        assert_eq!(result, "import ImportedClass from \"./MyCoolClass/ImportedClass\"
import ImportedClass2 from \"./MyCoolClass/ImportedClass2\"

export interface MyCoolType1 {
}

export interface MyCoolType2 {
}

export default class MyCoolClass {
    private server: any
    /**
        This method is used by easy-rpc internally and is not intended for manual use. It can be used to set the server of the object.
    */
    private setERPCServer(server: any) {
        this.server = server

        // trigger the setters to set the handlers on the server object
        if (this.MySuperCoolEndpoint1) {
            this.MySuperCoolEndpoint1 = this.MySuperCoolEndpoint1
        }
        if (this.MySuperCoolEndpoint2) {
            this.MySuperCoolEndpoint2 = this.MySuperCoolEndpoint2
        }
    }

    constructor(callbacks?: {
        MySuperCoolEndpoint1: (p1?: string[], p2: number) => Promise<string[]>
        MySuperCoolEndpoint2: () => Promise<void>
        ImportedClass: ImportedClass
        ImportedClass2: ImportedClass2
    }) {
        if (callbacks?.MySuperCoolEndpoint1) {
            this.MySuperCoolEndpoint1 = callbacks.MySuperCoolEndpoint1
        }

        if (callbacks?.MySuperCoolEndpoint2) {
            this.MySuperCoolEndpoint2 = callbacks.MySuperCoolEndpoint2
        }

        if (callbacks?.ImportedClass) {
            this.ImportedClass = callbacks.ImportedClass
        } else {
            this.ImportedClass = this.ImportedClass
        }

        if (callbacks?.ImportedClass2) {
            this.ImportedClass2 = callbacks.ImportedClass2
        } else {
            this.ImportedClass2 = this.ImportedClass2
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

    private _ImportedClass = new ImportedClass()
    set ImportedClass(value: ImportedClass) {
        this._ImportedClass = value;
        (value as any).setERPCServer(this.server)
    }
    get ImportedClass() {
        return this._ImportedClass
    }
    private _ImportedClass2 = new ImportedClass2()
    set ImportedClass2(value: ImportedClass2) {
        this._ImportedClass2 = value;
        (value as any).setERPCServer(this.server)
    }
    get ImportedClass2() {
        return this._ImportedClass2
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

        let result = generate_class(
            "MyCoolClass",
            "test/test2/",
            &endpoints,
            true,
            &vec!["ImportedClass".to_string(), "ImportedClass2".to_string()],
            &vec![
                CustomType {
                    documentation: None,
                    start: CodePosition::zero_initialized(),
                    end: CodePosition::zero_initialized(),
                    fields: vec![],
                    identifier: "MyCoolType1".to_string(),
                },
                CustomType {
                    documentation: None,
                    start: CodePosition::zero_initialized(),
                    end: CodePosition::zero_initialized(),
                    fields: vec![],
                    identifier: "MyCoolType2".to_string(),
                },
            ],
        );

        assert_eq!(
            result,
            "import ImportedClass from \"./MyCoolClass/ImportedClass\"
import ImportedClass2 from \"./MyCoolClass/ImportedClass2\"

export interface MyCoolType1 {
}

export interface MyCoolType2 {
}

export default class MyCoolClass {
    ImportedClass: ImportedClass
    ImportedClass2: ImportedClass2

    private server: any

    constructor(server: any) {
        this.server = server
        this.ImportedClass = new ImportedClass(server)
        this.ImportedClass2 = new ImportedClass2(server)
    }

/**some docs*/
    MySuperCoolEndpoint1(p1?: string[], p2: number): Promise<string[]> {
        return this.server.call(\"test/test2/MyCoolClass/MySuperCoolEndpoint1\", [p1, p2])
    }

    MySuperCoolEndpoint2(): Promise<void> {
        return this.server.call(\"test/test2/MyCoolClass/MySuperCoolEndpoint2\")
    }

}"
        );
    }
}
