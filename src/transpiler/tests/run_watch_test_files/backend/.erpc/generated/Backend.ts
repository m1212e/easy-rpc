import { ERPCServer, ServerOptions } from "@easy-rpc/node"
import Frontend from "./Frontend"
import api from "./Backend/api"

export default class Backend extends ERPCServer {
    private _api = undefined as any
    set api(value: api) {
        this._api = value;
        (value as any).setERPCServer(this)
    }
    get api() {
        return this._api
    }
    /**
        @param options The options to set for the easy-rpc object
        @param callbacks Callbacks to register for this server
    */
    constructor(options: ServerOptions, callbacks?: {
        api: api
    }) {
        super(options, ["http-server", ], true, "Backend")
        if (callbacks?.api) {
            this.api = callbacks.api
        } else {
            this.api = new api()
        }
    }

    onConnection(callback: (target: Frontend) => void) {
        // eslint-disable-next-line @typescript-eslint/ban-ts-comment
        // @ts-ignore
        super.onSocketConnection(({ role, client}) => {
            if (role == "Frontend") {
                const ret = new Frontend({} as any)
                // eslint-disable-next-line @typescript-eslint/ban-ts-comment
                // @ts-ignore
                ret.setERPCSocket(client)
                callback(ret)
            }
        })
    }
}