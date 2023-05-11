import Backend from "../.erpc/generated/Backend";
const port = 1234;
const backend = new Backend({
	allowedCorsOrigins: ["*"],
	port,
});

backend.api.ping = async (msg) => {
	console.log(`Got message from frontend: ${msg}`);

	return "PONG";
};

backend.onConnection(async (frontend) => {
	console.log("Frontend connected");

	console.log("returned from frontend: ", await frontend.api.ping("PING"));
});

backend.run();
console.log(`Running backend on port ${port}`);

