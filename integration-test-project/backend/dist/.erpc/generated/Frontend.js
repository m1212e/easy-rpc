"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
const node_1 = require("@easy-rpc/node");
const api_1 = __importDefault(require("./Frontend/api"));
class Frontend extends node_1.ERPCTarget {
    /**
        @param options The options to set for the easy-rpc object
    */
    constructor(options) {
        super(options, "browser");
        this.api = new api_1.default(this);
    }
}
exports.default = Frontend;
//# sourceMappingURL=Frontend.js.map