
export default class api {

    private server: any

    constructor(server: any) {
        this.server = server
    }

    ping(msg: string): Promise<string> {
        return this.server.call("api/ping", [msg])
    }

}