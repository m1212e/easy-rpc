import Backend from "../.erpc/generated/Backend";
import Frontend from "../.erpc/generated/Frontend";


const backend = new Backend({
	address: "http://localhost:1234",
});
console.log(await backend.api.ping("This message is from frontend"));

const frontend = new Frontend({});
frontend.api.ping = async (message) => {
	console.log(`got from backed: ${message}`);
	return "PONG";
};
frontend.run();
