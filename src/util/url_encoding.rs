pub fn decode(value: &str) -> String {
    value
        .replace('+', " ")
        .replace("%20", " ")
        .replace("%21", "!")
        .replace("%22", "\"")
        .replace("%23", "#")
        .replace("%24", "$")
        .replace("%25", "%")
        .replace("%26", "&")
        .replace("%27", "'")
        .replace("%28", "(")
        .replace("%29", ")")
        .replace("%2A", "*")
        .replace("%2a", "*")
        .replace("%2B", "+")
        .replace("%2b", "+")
        .replace("%2C", ",")
        .replace("%2c", ",")
        .replace("%2F", "/")
        .replace("%2f", "/")
        .replace("%3A", ":")
        .replace("%3a", ":")
        .replace("%3B", ";")
        .replace("%3b", ";")
        .replace("%3D", "=")
        .replace("%3d", "=")
        .replace("%3F", "?")
        .replace("%3f", "?")
        .replace("%40", "@")
        .replace("%5B", "[")
        .replace("%5b", "[")
        .replace("%5D", "]")
        .replace("%5d", "]")
}
