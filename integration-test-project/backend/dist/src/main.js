"use strict";
var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
const Backend_1 = __importDefault(require("../.erpc/generated/Backend"));
const port = 1234;
const backend = new Backend_1.default({
    allowedCorsOrigins: ["*"],
    port,
});
backend.api.ping = (msg) => __awaiter(void 0, void 0, void 0, function* () {
    console.log(`Got message from frontend: ${msg}`);
    return "PONG";
});
backend.run();
console.log(`Running backend on port ${port}`);
setTimeout(() => {
    backend.onConnection((frontend) => __awaiter(void 0, void 0, void 0, function* () {
        console.log("Frontend connected");
        console.log("returned from frontend: ", yield frontend.api.ping("PING"));
    }));
}, 1000);
//# sourceMappingURL=main.js.map