import Backend from "../.erpc/generated/Backend";

document.querySelector<HTMLDivElement>("#app")!.innerHTML = `
  <button id="makecallbutton">make call</button>
`;

const backend = new Backend({
  address: "http://localhost",
  port: 1234,
});

const btn = document.getElementById("makecallbutton");

btn!.addEventListener("click", async () => {
  console.log(await backend.api.login("something"));
});
