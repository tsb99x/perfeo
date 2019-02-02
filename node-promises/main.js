const http = require('http')
const MongoClient = require('mongodb').MongoClient

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

function insert_one_record (db) {
  return new Promise((resolve, reject) => {
    if (keepsHisWord) {
      resolve('The man likes to keep his word')
    } else {
      reject('The man does not want to keep his word')
    }
  })
}

const port = getEnv('PORT')
const dbUri = getEnv('DB_URI')

const client = new MongoClient(dbUri)
client.connect((err) => {
  if (err) throw err
  const db = client.db('test')
  console.log('Connected to database')

  const server = http.createServer((req, res) => {
    if (req.method === 'POST' && req.url === '/') {
      let body = ''
      req.on('data', (chunk) => {
        body += chunk
      })
      req.on('end', () => {
        try {
          const json = JSON.parse(body)

          db.collection('test').insertOne(json, (err, object) => {
            if (err) {
              sendPlainText(res, 500, 'INTERNAL_SERVER_ERROR')
            } else {
              sendPlainText(res, 200, 'OK')
            }
          })
        } catch (e) {
          sendPlainText(res, 400, 'BAD_REQUEST')
        }
      })
    } else {
      sendPlainText(res, 404, 'NOT_FOUND')
    }
  })

  server.listen(port, () => {
    console.log(`Server launched on ${port}`)
  })
})
