import { ERPCServer, ServerOptions } from "@easy-rpc/node"
import Client from "./Client"
import api from "./Server/api"
import auth from "./Server/auth"

/**This is some docs*/
export default class Server extends ERPCServer {
    private _api = undefined as any
    set api(value: api) {
        this._api = value;
        (value as any).setERPCServer(this)
    }
    get api() {
        return this._api
    }
    private _auth = undefined as any
    set auth(value: auth) {
        this._auth = value;
        (value as any).setERPCServer(this)
    }
    get auth() {
        return this._auth
    }
    /**
        @param options The options to set for the easy-rpc object
        @param callbacks Callbacks to register for this server
    */
    constructor(options: ServerOptions, callbacks?: {
        api: api
        auth: auth
    }) {
        super(options, "http-server", true, "Server")
        if (callbacks?.api) {
            this.api = callbacks.api
        } else {
            this.api = new api()
        }
        if (callbacks?.auth) {
            this.auth = callbacks.auth
        } else {
            this.auth = new auth()
        }
    }

    onConnection(callback: (target: Client) => void) {
        // eslint-disable-next-line @typescript-eslint/ban-ts-comment
        // @ts-ignore
        super.onSocketConnection((role, socket) => {
            if (role === "Client") {
                const ret = new Client({address: ""})
                // eslint-disable-next-line @typescript-eslint/ban-ts-comment
                // @ts-ignore
                ret.setERPCSocket(socket)
                callback(ret)
            }
        })
    }
}