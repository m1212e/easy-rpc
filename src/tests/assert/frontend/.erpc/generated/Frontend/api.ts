
export default class api {
    private server: any
    /**
        This method is used by easy-rpc internally and is not intended for manual use. It can be used to set the server of the object.
    */
    private setERPCServer(server: any) {
        this.server = server

        // trigger the setters to set the handlers on the server object
        if (this.login2) {
            this.login2 = this.login2
        }
    }

    constructor(callbacks?: {
        login2: (newUser: string) => Promise<"success">
    }) {
        if (callbacks?.login2) {
            this.login2 = callbacks.login2
        }

    }

    private _login2: (newUser: string) => Promise<"success"> = undefined as any
    set login2(value: (newUser: string) => Promise<"success">) {
        this._login2 = value
        this.server?.registerERPCCallbackFunction(value, "api/login2")
    }
    get login2() {
        return this._login2
    }


}