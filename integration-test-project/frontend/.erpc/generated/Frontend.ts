import { ERPCServer, ServerOptions } from "@easy-rpc/browser"
import api from "./Frontend/api"

export default class Frontend extends ERPCServer {
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
        super(options, "browser", true, "Frontend")
        if (callbacks?.api) {
            this.api = callbacks.api
        } else {
            this.api = new api()
        }
    }

    // private free() {
    //     super.free()
    // }
}