const fs = require("fs");

if (!(fs.existsSync("data/configs.json"))) {
    fs.writeFileSync(
        "data/configs.json",
        fs.readFileSync("data/defaultConfigs.json", { encoding: "utf8", flag: "r" })
    );
}
const settings = JSON.parse(fs.readFileSync("data/configs.json", { encoding: "utf8", flag: "r" }));
const language = JSON.parse(fs.readFileSync(`data/language/${settings.language}.json`, { encoding: "utf8", flag: "r" }));

async function writeText() {
    while ( await new Promise(r => setTimeout(() => r(1), 200)) ) {

        for (i in language) {
            var t = document.getElementsByClassName("text-"+i);
            for (var j = 0; j < t.length; j++) {
                if (t[j].innerText != language[i])
                    t[j].innerText = language[i];
            }
        }
    }
}