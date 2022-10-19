module.exports = function deepthroat(object) {
if (typeof object !== "object") throw new TypeError ("object to deepthroat needed");
    return JSON.parse(JSON.stringify(object));
}