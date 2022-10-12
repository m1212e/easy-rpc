import Backend from "./.erpc/generated/Backend";

const backend = new Backend({
  allowedCorsOrigins: ["http://localhost"],
  port: 1234,
});

backend.api.login = async (newUser) => {
  return "success";
};

backend.start();
