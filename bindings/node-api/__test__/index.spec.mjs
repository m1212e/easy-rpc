import test from 'ava'
import {ERPCServer, ERPCTarget} from '../index.js'

//TODO test sockets
//TODO test more data types and constellations

test('test creation', (t) => {

  const server = new ERPCServer({
    port: 9988,
    allowedCorsOrigins: ["*"]
  }, "http-server", true, "Backend");

  const target = new ERPCTarget({
    address: "",
    port: 0
  }, "http-server")

  t.assert(server !== undefined)
  t.assert(target !== undefined)
})

test('test handler calls', async (t) => {
  const server = new ERPCServer({
    port: 9988,
    allowedCorsOrigins: ["*"]
  }, "http-server", true, "Backend");

  server.registerERPCHandler((p1, p2, p3, p4, p5, p6) => {
    t.deepEqual(p1, "p1")
    t.deepEqual(p2, 17)
    t.deepEqual(p3, -17)
    t.deepEqual(p4, -17.6)
    t.deepEqual(p5, true)
    t.deepEqual(p6, {a: 17})

    return "helllloooo"
  }, "some/handler/identifier")

  server.registerERPCHandler(() => {
  }, "some/handler/identifier/two")


  setTimeout(() => {
    server.stop();
  }, 5000);

  const target = new ERPCTarget({
    address: "http://localhost",
    port: 9988
  }, "http-server")

  setTimeout(async () => {
    let r = await target.call("some/handler/identifier", ["p1", 17, -17, -17.6, true, {a: 17}])
    t.assert(r === "helllloooo")

    let r2 = await target.call("some/handler/identifier/two")
    t.assert(!r2)
  }, 1000);

  await server.run();
})