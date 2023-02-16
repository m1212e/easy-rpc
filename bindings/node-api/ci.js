const fs = require("fs");
const {exec} = require("child_process");

const pkgjson = JSON.parse(fs.readFileSync("./package.json"));
pkgjson.version = process.env.REF_NAME;
fs.writeFileSync("./package.json", JSON.stringify(pkgjson))

exec("npm run version", (error, stdout, stderr) => {
    if (error) {
        console.log(`error: ${error.message}`);
        return;
    }
    if (stderr) {
        console.log(`stderr: ${stderr}`);
        return;
    }
    console.log(`stdout: ${stdout}`);
});