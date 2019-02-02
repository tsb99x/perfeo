const express = require('express')
const MongoClient = require('mongodb').MongoClient

function getEnv (varName) {
  const res = process.env[varName]
  if (!res) throw Error(`Failed to get '${varName}' env variable!`)
  return res
}

const port = getEnv('PORT')
const dbUri = getEnv('DB_URI')

const client = new MongoClient(dbUri)
client.connect((err) => {
  if (err) throw err
  const db = client.db('test')

  const app = express()

  app.use(express.json())

  app.post('/', (req, res) => {
    if (!req.is('json')) {
      res.sendStatus(400)
    } else {
      db.collection('test').insertOne(req.body, (err, object) => {
        if (err) {
          throw err
        } else {
          res.sendStatus(200)
        }
      })
    }
  })

  app.listen(port, () => {
    console.log(`Server is up on port ${port}`)
  })
})
