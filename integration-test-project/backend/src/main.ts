import Backend from "../.erpc/generated/Backend";
const port = 1234;
const backend = new Backend({
	allowedCorsOrigins: ["*"],
	port,
});

backend.api.ping = async (msg) => {
	return msg;
};

backend.onConnection(async (frontend) => {
	console.log("Frontend connected");
	setTimeout(async () => {
		console.log("sending ping request");
		console.log("returned from frontend: ", await frontend.api.ping("This message is from backend"));
	}, 2000);
});

backend.run();
console.log(`Running backend on port ${port}`);
