#[cfg(test)]
mod tests {
    use crate::transpiler::{
        config::Role, generator::translator::typescript::client::generate_client,
    };

    #[test]
    fn test_success_foreign() {
        let result = generate_client(
            true,
            &vec!["api".to_string(), "tracks".to_string()],
            &Role {
                documentation: Some("Example docs".to_string()),
                name: "Client".to_string(),
                types: vec!["browser".to_string()],
            },
            &vec!["Client".to_string()],
            "@easy-rpc/browser",
        );

        assert_eq!(
            result,
            "import { ERPCTarget, TargetOptions } from \"@easy-rpc/browser\"
import api from \"./Client/api\"
import tracks from \"./Client/tracks\"

/**Example docs*/
export default class Client extends ERPCTarget {
    api = new api(this)
    tracks = new tracks(this)
    /**
        @param options The options to set for the easy-rpc object
    */
    constructor(options: TargetOptions) {
        super(options, [\"browser\", ])
    }
}"
        );
    }

    #[test]
    fn test_success_callback() {
        let result = generate_client(
            false,
            &vec!["api".to_string(), "tracks".to_string()],
            &Role {
                documentation: Some("Example docs".to_string()),
                name: "Server".to_string(),
                types: vec!["http-server".to_string()],
            },
            &vec!["Client".to_string()],
            "@easy-rpc/node",
        );

        assert_eq!(
            result,
            "import { ERPCServer, ServerOptions } from \"@easy-rpc/node\"
import Client from \"./Client\"
import api from \"./Server/api\"
import tracks from \"./Server/tracks\"

/**Example docs*/
export default class Server extends ERPCServer {
    private _api = undefined as any
    set api(value: api) {
        this._api = value;
        (value as any).setERPCServer(this)
    }
    get api() {
        return this._api
    }
    private _tracks = undefined as any
    set tracks(value: tracks) {
        this._tracks = value;
        (value as any).setERPCServer(this)
    }
    get tracks() {
        return this._tracks
    }
    /**
        @param options The options to set for the easy-rpc object
        @param callbacks Callbacks to register for this server
    */
    constructor(options: ServerOptions, callbacks?: {
        api: api
        tracks: tracks
    }) {
        super(options, [\"http-server\", ], true, \"Server\")
        if (callbacks?.api) {
            this.api = callbacks.api
        } else {
            this.api = new api()
        }
        if (callbacks?.tracks) {
            this.tracks = callbacks.tracks
        } else {
            this.tracks = new tracks()
        }
    }

    onConnection(callback: (target: Client) => void) {
        // eslint-disable-next-line @typescript-eslint/ban-ts-comment
        // @ts-ignore
        super.onSocketConnection((role, socket) => {
            if (role == \"Client\") {
                const ret = new Client({address: \"\", port: 0})
                // eslint-disable-next-line @typescript-eslint/ban-ts-comment
                // @ts-ignore
                ret.setERPCSocket(socket)
                callback(ret)
            }
        })
    }
}"
        );
    }
}

//TODO write some tests whith variation (no docs, no return type etc.)
