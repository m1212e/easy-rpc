const fs = require('fs');
const path = require('path');

const GENERATED_PACKAGEJSON_PATH = path.join(__dirname, "pkg", "package.json");

if (fs.existsSync(GENERATED_PACKAGEJSON_PATH)) {
    const pkgjson = JSON.parse(fs.readFileSync(GENERATED_PACKAGEJSON_PATH));
    if (process.env.REF_NAME) {
        pkgjson.version = process.env.REF_NAME
    }
    pkgjson.name = "@easy-rpc/browser"
    fs.writeFileSync(GENERATED_PACKAGEJSON_PATH, JSON.stringify(pkgjson))
}

const GENERATED_TYPES_PATH = path.join(__dirname, "pkg", "browser.d.ts");

const generated_types = fs.readFileSync(GENERATED_TYPES_PATH);
// remove the wasm-bindgen generated free() method, which can be omitted from the generated types since its usage is not
// necessary for the end user
fs.writeFileSync(GENERATED_TYPES_PATH, generated_types.toString().replaceAll("  free(): void;\n", ""))