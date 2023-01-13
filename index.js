module.exports = function deepthroat(object) {
    if (typeof object == "number" || object == null || object == true || object == false || object instanceof Function) {
        return object;
    }
    var output = new object.constructor();
    for (var i in object) {
        output[i] = module.exports(object[i]);
    }
    return output;
}
