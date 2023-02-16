import { ERPCTarget, TargetOptions } from "@easy-rpc/node"
import api from "./Client/api"
import auth from "./Client/auth"

/**This is some docs*/
export default class Client extends ERPCTarget {
    api = new api(this)
    auth = new auth(this)
    /**
        @param options The options to set for the easy-rpc object
    */
    constructor(options: TargetOptions) {
        super(options, "browser")
    }
}