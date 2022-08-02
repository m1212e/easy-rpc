
export default class time {

    private server: any

    constructor(server: any) {
        this.server = server
    }

    test7(): Promise<void> {
        return this.server.call("api\/time/test7")
    }

}