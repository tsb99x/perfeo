const http = require('http')
const MongoClient = require('mongodb').MongoClient
const worker = require('cluster').worker
const log = require('debug')(`worker:${worker.id}`)

const { getEnv, sendPlainText } = require('./utils')

const port = getEnv('PORT')
const dbUri = getEnv('DB_URI')

const client = new MongoClient(dbUri, { useNewUrlParser: true })
client.connect((err) => {
    if (err) throw err
    const db = client.db('test')
    log('Connected to database')

    const server = http.createServer((req, res) => {
        if (req.method === 'GET' && req.url === '/ping') {
            sendPlainText(res, 200, 'PONG')
        } else if (req.method === 'POST' && req.url === '/') {
            let body = ''
            req.on('data', (chunk) => {
                body += chunk
            })

            req.on('end', () => {
                let json
                
                try {
                    json = JSON.parse(body)
                } catch (e) {
                    sendPlainText(res, 400, 'BAD_REQUEST')
                }

                db.collection('test').insertOne(json, (err) => {
                    if (err) {
                        sendPlainText(res, 500, 'INTERNAL_SERVER_ERROR')
                    } else {
                        sendPlainText(res, 200, 'OK')
                    }
                })
            })
        } else {
            sendPlainText(res, 404, 'NOT_FOUND')
        }
    })

    server.listen(port, () => {
        log(`Server launched on ${port}`)
    })
})
