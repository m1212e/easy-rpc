"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
class api {
    constructor(server) {
        this.server = server;
    }
    ping(msg) {
        return this.server.call("api/ping", [msg]);
    }
}
exports.default = api;
//# sourceMappingURL=api.js.map