
export default class api {

    private server: any

    constructor(server: any) {
        this.server = server
    }

    test8(): Promise<void> {
        return this.server.call("api/test8")
    }

}