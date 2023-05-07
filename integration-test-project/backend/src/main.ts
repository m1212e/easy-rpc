import Backend from "../.erpc/generated/Backend";
const backend = new Backend({
	allowedCorsOrigins: ["*"],
	port: 1234,
});

backend.api.ping = async (msg) => {
	return "PONG";
};

backend.run();
