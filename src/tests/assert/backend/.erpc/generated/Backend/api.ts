
export default class api {
    private server: any
    /**
        This method is used by easy-rpc internally and is not intended for manual use. It can be used to set the server of the object.
    */
    private setERPCServer(server: any) {
        this.server = server

        // trigger the setters to set the handlers on the server object
        this.login = this.login
    }

    constructor(callbacks?: {
        login: (newUser: string) => "success" | "fail" | Promise<"success" | "fail">
    }) {
        if (callbacks?.login) {
            this.login = callbacks.login
        }

    }

    private _login: (newUser: string) => "success" | "fail" | Promise<"success" | "fail"> = undefined as any
    set login(value: (newUser: string) => "success" | "fail" | Promise<"success" | "fail">) {
        this._login = value
        this.server?.registerERPCCallbackFunction(value, "api/login")
    }
    get login() {
        return this._login
    }


}