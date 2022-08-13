
export default class models {

    private server: any

    constructor(server: any) {
        this.server = server
    }

    test8(): Promise<void> {
        return this.server.call("api/roles/models/test8")
    }

}