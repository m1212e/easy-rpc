import tracks from "./api/tracks"

export default class api {
    private server: any
    /**
        This method is used by easy-rpc internally and is not intended for manual use. It can be used to set the server of the object.
    */
    private setERPCServer(server: any) {
        this.server = server

        // trigger the setters to set the handlers on the server object
        this.test3 = this.test3
        this.test4 = this.test4
    }

    constructor(callbacks?: {
        test3: () => Promise<void>
        test4: () => Promise<void>
        tracks: tracks
    }) {
        if (callbacks?.test3) {
            this.test3 = callbacks.test3
        }

        if (callbacks?.test4) {
            this.test4 = callbacks.test4
        }

        if (callbacks?.tracks) {
            this.tracks = callbacks.tracks
        } else {
            this.tracks = this.tracks
        }

    }

    private _test3: () => Promise<void> = undefined as any
    set test3(value: () => Promise<void>) {
        this._test3 = value
        this.server?.registerERPCCallbackFunction(value, "/api/test3")
    }
    get test3() {
        return this._test3
    }

    private _test4: () => Promise<void> = undefined as any
    set test4(value: () => Promise<void>) {
        this._test4 = value
        this.server?.registerERPCCallbackFunction(value, "/api/test4")
    }
    get test4() {
        return this._test4
    }

    private _tracks = new tracks()
    set tracks(value: tracks) {
        this._tracks = value;
        (value as any).setERPCServer(this.server)
    }
    get tracks() {
        return this._tracks
    }

}