import time from "./api/time"

export default class api {
    time: time

    private server: any

    constructor(server: any) {
        this.server = server
        this.time = new time(server)
    }

    test5(): Promise<void> {
        return this.server.call("/api/test5")
    }

}