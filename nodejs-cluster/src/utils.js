function getEnv(varName, defaultValue) {
    const res = process.env[varName];
    return res ?? defaultValue;
}

function sendPlainText(res, statusCode, plainTextMsg) {
    res.statusCode = statusCode;
    res.setHeader("Content-Type", "text/plain");
    res.end(plainTextMsg);
}

module.exports = {
    getEnv,
    sendPlainText,
};
