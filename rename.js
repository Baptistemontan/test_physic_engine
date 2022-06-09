const fs = require("fs");

function main() {
    const file = fs.readFileSync("./build/index.html", "utf8");

    const newFile = file.replace(/href="(.*)"/g, (match, p1) => {
        return `href="/test_physic_engine${p1}"`;
    }).replace(/'([^']*)'/g, (match, p1) => {
        return `'/test_physic_engine${p1}'`;
    });

    fs.writeFileSync("./build/index.html", newFile);
}

main();