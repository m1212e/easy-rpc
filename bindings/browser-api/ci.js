const pkgjson = JSON.parse(fs.readFileSync("./pkg/package.json"));
if (process.env.REF_NAME) {
    pkgjson.version = process.env.REF_NAME
    pkgjson.name = "@easy-rpc/browser"
}
fs.writeFileSync("./pkg/package.json", JSON.stringify(pkgjson))