import cluster from "cluster";
import os from "os";
import debugFactory from "debug";
import main from "./main.js";

const numCPUs = os.availableParallelism();
const log = debugFactory("master");

if (cluster.isPrimary) {
    log(`Master [PID:${process.pid}] is running`);

    for (let i = 0; i < numCPUs; i++) {
        cluster.fork();
    }

    cluster.on("exit", (worker) => {
        log(`Worker [PID:${worker.process.pid}] is offline, restarting...`);
        cluster.fork();
    });
} else {
    main();
}
