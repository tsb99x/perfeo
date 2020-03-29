function getEnv (varName) {
    const res = process.env[varName]
    if (!res) throw Error(`Failed to get '${varName}' env variable!`)
    return res
}

function sendPlainText (res, statusCode, plainTextMsg) {
    res.statusCode = statusCode
    res.setHeader('Content-Type', 'text/plain')
    res.end(plainTextMsg)
}

module.exports = {
    getEnv,
    sendPlainText
}
