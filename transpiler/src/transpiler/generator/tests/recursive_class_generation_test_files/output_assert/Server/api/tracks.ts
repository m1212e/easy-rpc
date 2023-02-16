
export default class tracks {
    private server: any
    /**
        This method is used by easy-rpc internally and is not intended for manual use. It can be used to set the server of the object.
    */
    private setERPCServer(server: any) {
        this.server = server

        // trigger the setters to set the handlers on the server object
        if (this.test6) {
            this.test6 = this.test6
        }
    }

    constructor(callbacks?: {
        test6: () => Promise<void>
    }) {
        if (callbacks?.test6) {
            this.test6 = callbacks.test6
        }

    }

    private _test6: () => Promise<void> = undefined as any
    set test6(value: () => Promise<void>) {
        this._test6 = value
        this.server?.registerERPCHandler(value, "api/tracks/test6")
    }
    get test6() {
        return this._test6
    }


}