import { ERPCTarget, TargetOptions } from "@easy-rpc/node"
import api from "./Frontend/api"

export default class Frontend extends ERPCTarget {
    api = new api(this)
    /**
        @param options The options to set for the easy-rpc object
    */
    constructor(options: TargetOptions) {
        super(options, ["browser", ])
    }
}