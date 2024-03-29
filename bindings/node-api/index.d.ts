/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export interface ServerOptions {
  port: number
  allowedCorsOrigins: Array<string>
}
export interface TargetOptions {
  address: string
}
export class ERPCServer {
  constructor(options: ServerOptions, serverType: string, enableSockets: boolean, role: string)
  /**
  Starts the server as configured
  */
  run(): Promise<void>
  /**
  * Stops the server
  */
  stop(): void
}
export class ERPCTarget {
  constructor(options: TargetOptions, targetType: string)
}
