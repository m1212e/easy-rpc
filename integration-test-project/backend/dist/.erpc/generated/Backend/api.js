"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
class api {
    /**
        This method is used by easy-rpc internally and is not intended for manual use. It can be used to set the server of the object.
    */
    setERPCServer(server) {
        this.server = server;
        // trigger the setters to set the handlers on the server object
        if (this.ping) {
            this.ping = this.ping;
        }
    }
    constructor(callbacks) {
        this._ping = undefined;
        if (callbacks === null || callbacks === void 0 ? void 0 : callbacks.ping) {
            this.ping = callbacks.ping;
        }
    }
    set ping(value) {
        var _a;
        this._ping = value;
        (_a = this.server) === null || _a === void 0 ? void 0 : _a.registerERPCHandler(value, "api/ping");
    }
    get ping() {
        return this._ping;
    }
}
exports.default = api;
//# sourceMappingURL=api.js.map