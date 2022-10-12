
export default class api {

    private server: any

    constructor(server: any) {
        this.server = server
    }

    login2(newUser: string): Promise<"success"> {
        return this.server.call("api/login2", [newUser])
    }

}