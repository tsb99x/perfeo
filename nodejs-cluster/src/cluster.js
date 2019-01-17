const cluster = require('cluster')
const numCPUs = require('os').cpus().length
const log = require('debug')('master')

if (cluster.isMaster) {
    log(`Master [PID:${process.pid}] is running`)

    for (let i = 0; i < numCPUs; i++) {
        cluster.fork()
    }

    cluster.on('exit', (worker) => {
        log(`Worker [PID:${worker.process.pid}] is offline`)
    })
} else {
    require('./main')
}
