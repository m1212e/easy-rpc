
export default class auth {
    private server: any
    /**
        This method is used by easy-rpc internally and is not intended for manual use. It can be used to set the server of the object.
    */
    private setERPCServer(server: any) {
        this.server = server

        // trigger the setters to set the handlers on the server object
        if (this.test1) {
            this.test1 = this.test1
        }
    }

    constructor(callbacks?: {
        test1: () => Promise<void>
    }) {
        if (callbacks?.test1) {
            this.test1 = callbacks.test1
        }

    }

    private _test1: () => Promise<void> = undefined as any
    set test1(value: () => Promise<void>) {
        this._test1 = value
        this.server?.registerERPCCallbackFunction(value, "auth/test1")
    }
    get test1() {
        return this._test1
    }


}