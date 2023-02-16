
export default class api {

    private server: any

    constructor(server: any) {
        this.server = server
    }

    test1(): Promise<void> {
        return this.server.call("api/test1")
    }

}