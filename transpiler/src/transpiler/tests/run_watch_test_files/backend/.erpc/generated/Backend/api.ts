
export default class api {
    private server: any
    /**
        This method is used by easy-rpc internally and is not intended for manual use. It can be used to set the server of the object.
    */
    private setERPCServer(server: any) {
        this.server = server

        // trigger the setters to set the handlers on the server object
        if (this.test2) {
            this.test2 = this.test2
        }
    }

    constructor(callbacks?: {
        test2: () => Promise<void>
    }) {
        if (callbacks?.test2) {
            this.test2 = callbacks.test2
        }

    }

    private _test2: () => Promise<void> = undefined as any
    set test2(value: () => Promise<void>) {
        this._test2 = value
        this.server?.registerERPCCallbackFunction(value, "api/test2")
    }
    get test2() {
        return this._test2
    }


}