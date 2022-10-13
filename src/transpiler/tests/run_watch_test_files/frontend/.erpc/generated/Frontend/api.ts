
export default class api {
    private server: any
    /**
        This method is used by easy-rpc internally and is not intended for manual use. It can be used to set the server of the object.
    */
    private setERPCServer(server: any) {
        this.server = server

        // trigger the setters to set the handlers on the server object
        this.test1 = this.test1
        this.test7 = this.test7
        this.test8 = this.test8
        this.test9 = this.test9
    }

    constructor(callbacks?: {
        test1: () => void | Promise<void>
        test7: () => void | Promise<void>
        test8: () => void | Promise<void>
        test9: () => void | Promise<void>
    }) {
        if (callbacks?.test1) {
            this.test1 = callbacks.test1
        }

        if (callbacks?.test7) {
            this.test7 = callbacks.test7
        }

        if (callbacks?.test8) {
            this.test8 = callbacks.test8
        }

        if (callbacks?.test9) {
            this.test9 = callbacks.test9
        }

    }

    private _test1: () => void | Promise<void> = undefined as any
    set test1(value: () => void | Promise<void>) {
        this._test1 = value
        this.server?.registerERPCCallbackFunction(value, "api/test1")
    }
    get test1() {
        return this._test1
    }

    private _test7: () => void | Promise<void> = undefined as any
    set test7(value: () => void | Promise<void>) {
        this._test7 = value
        this.server?.registerERPCCallbackFunction(value, "api/test7")
    }
    get test7() {
        return this._test7
    }

    private _test8: () => void | Promise<void> = undefined as any
    set test8(value: () => void | Promise<void>) {
        this._test8 = value
        this.server?.registerERPCCallbackFunction(value, "api/test8")
    }
    get test8() {
        return this._test8
    }

    private _test9: () => void | Promise<void> = undefined as any
    set test9(value: () => void | Promise<void>) {
        this._test9 = value
        this.server?.registerERPCCallbackFunction(value, "api/test9")
    }
    get test9() {
        return this._test9
    }


}