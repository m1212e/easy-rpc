
export default class auth {

    private server: any

    constructor(server: any) {
        this.server = server
    }

    test2(): Promise<void> {
        return this.server.call("/auth/test2")
    }

}