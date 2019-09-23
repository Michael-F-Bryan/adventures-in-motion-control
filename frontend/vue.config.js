const path = require("path");

module.exports = {
    configureWebpack: config => {
        config.entry.app = path.join(__dirname, "src", "bootstrap.js");
    }
}
