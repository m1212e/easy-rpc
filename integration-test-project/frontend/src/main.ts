import Backend from "../.erpc/generated/Backend";
import Frontend from "../.erpc/generated/Frontend";

setTimeout(() => {
	const backend = new Backend({
		address: "http://localhost:1234",
	});
}, 1000);

// const returned = await backend.api.ping("hello");
// console.log(`returned from backed: ${returned}`);

const frontend = new Frontend({});
frontend.api.ping = async (message) => {
	console.log(`got from backed: ${message}`);
	return "PONG";
};
frontend.run();
