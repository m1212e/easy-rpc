The parser module is used to parse a text into data stuctures which can be worked on by the rest of the transpiler.

### input_reader
[input_reader](./input_reader/) is used as a wrapper around the rust Read interface which describes a readable source of chars/bytes. It provides some helpful methods for parsing through an input. When parsing input this is the lowest level of abstraction and the first step in processing the input.

### lexer
[lexer](./lexer/) lexes the content of a provided input_reader into tokens. Tokens are the building blocks of a full syntax and are the second step in parsing an input.

### parser
[parser](./parser/) parses tokens into actually useable data structures.


## Note about parse return types
The parser and its submodules often use a pattern of ``Option<Result<XYZ, ParseError>>`` as return values. The option tells if the called function was confident that the currently parsed input should be handled. E.g. the ``parse_endpoint`` function actually tried to parse the endpoint and detected that an endpoint is being parsed. The result on the other hand provides either the parsed data or an error which occurred while parsing.