
export default class api {

    private server: any

    constructor(server: any) {
        this.server = server
    }

    login(newUser: string): Promise<"success" | "fail"> {
        return this.server.call("api/login", [newUser])
    }

}