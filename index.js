deepthroat = (object) => typeof object == "number" || object == null || object instanceof Function ? object : (new object.constructor()).map((input) => module.exports(input))

module.exports = deepthroat