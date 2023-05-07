
export default class api {
    private server: any
    /**
        This method is used by easy-rpc internally and is not intended for manual use. It can be used to set the server of the object.
    */
    private setERPCServer(server: any) {
        this.server = server

        // trigger the setters to set the handlers on the server object
        if (this.ping) {
            this.ping = this.ping
        }
    }

    constructor(callbacks?: {
        ping: (msg: string) => Promise<string>
    }) {
        if (callbacks?.ping) {
            this.ping = callbacks.ping
        }

    }

    private _ping: (msg: string) => Promise<string> = undefined as any
    set ping(value: (msg: string) => Promise<string>) {
        this._ping = value
        this.server?.registerERPCHandler(value, "api/ping")
    }
    get ping() {
        return this._ping
    }


}