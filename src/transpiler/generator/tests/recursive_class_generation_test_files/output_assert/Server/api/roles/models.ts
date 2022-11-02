
export default class models {
    private server: any
    /**
        This method is used by easy-rpc internally and is not intended for manual use. It can be used to set the server of the object.
    */
    private setERPCServer(server: any) {
        this.server = server

        // trigger the setters to set the handlers on the server object
        if (this.test9) {
            this.test9 = this.test9
        }
    }

    constructor(callbacks?: {
        test9: () => Promise<void>
    }) {
        if (callbacks?.test9) {
            this.test9 = callbacks.test9
        }

    }

    private _test9: () => Promise<void> = undefined as any
    set test9(value: () => Promise<void>) {
        this._test9 = value
        this.server?.registerERPCCallbackFunction(value, "api/roles/models/test9")
    }
    get test9() {
        return this._test9
    }


}