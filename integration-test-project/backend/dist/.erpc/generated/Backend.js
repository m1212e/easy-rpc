"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
const node_1 = require("@easy-rpc/node");
const Frontend_1 = __importDefault(require("./Frontend"));
const api_1 = __importDefault(require("./Backend/api"));
class Backend extends node_1.ERPCServer {
    set api(value) {
        this._api = value;
        value.setERPCServer(this);
    }
    get api() {
        return this._api;
    }
    /**
        @param options The options to set for the easy-rpc object
        @param callbacks Callbacks to register for this server
    */
    constructor(options, callbacks) {
        super(options, "http-server", true, "Backend");
        this._api = undefined;
        if (callbacks === null || callbacks === void 0 ? void 0 : callbacks.api) {
            this.api = callbacks.api;
        }
        else {
            this.api = new api_1.default();
        }
    }
    onConnection(callback) {
        // eslint-disable-next-line @typescript-eslint/ban-ts-comment
        // @ts-ignore
        super.onSocketConnection((role, socket) => {
            if (role == "Frontend") {
                const ret = new Frontend_1.default({ address: "", port: 0 });
                // eslint-disable-next-line @typescript-eslint/ban-ts-comment
                // @ts-ignore
                ret.setERPCSocket(socket);
                callback(ret);
            }
        });
    }
}
exports.default = Backend;
//# sourceMappingURL=Backend.js.map