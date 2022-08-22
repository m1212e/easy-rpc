import { ERPCTarget, TargetOptions } from "@easy-rpc/browser"
import Frontend from "./Frontend"
import api from "./Backend/api"

export default class Backend extends ERPCTarget {
    api = new api(this)
    /**
        @param options The options to set for the easy-rpc object
    */
    constructor(options: TargetOptions) {
        super(options, ["http-server", ])
    }
}