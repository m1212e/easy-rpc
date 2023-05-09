const fs = require('fs');
const path = require('path');

const GENERATED_PACKAGEJSON_PATH = path.join(__dirname, "pkg", "package.json");

if (fs.existsSync(GENERATED_PACKAGEJSON_PATH)) {
    const pkgjson = JSON.parse(fs.readFileSync(GENERATED_PACKAGEJSON_PATH));
    if (process.env.REF_NAME) {
        pkgjson.version = process.env.REF_NAME
    }
    pkgjson.name = "@easy-rpc/browser"
    fs.writeFileSync("./pkg/package.json", JSON.stringify(pkgjson))
}