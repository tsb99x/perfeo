import http from "http";
import { MongoClient } from "mongodb";
import cluster from "cluster";
import debugFactory from "debug";

const port = getEnv("PORT", 8080);
const dbUri = getEnv("DB_URI", "mongodb://localhost:27017");

function getEnv(varName, defaultValue) {
    const res = process.env[varName];
    return res ?? defaultValue;
}

function sendPlainText(res, statusCode, plainTextMsg) {
    res.statusCode = statusCode;
    res.setHeader("Content-Type", "text/plain");
    res.end(plainTextMsg);
}

export default function () {
    const log = debugFactory(`worker:${cluster.worker.id}`);

    const client = new MongoClient(dbUri);
    const db = client.db("test");
    const collection = db.collection("test");

    const server = http.createServer((req, res) => {
        if (req.method === "GET" && req.url === "/ping") {
            sendPlainText(res, 200, "PONG");
        } else if (req.method === "POST" && req.url === "/") {
            let body = "";
            req.on("data", (chunk) => {
                body += chunk;
            });

            req.on("end", async () => {
                let json;

                try {
                    json = JSON.parse(body);
                } catch {
                    sendPlainText(res, 400, "BAD_REQUEST");
                    return;
                }

                try {
                    await collection.insertOne(json);
                } catch {
                    sendPlainText(res, 500, "INTERNAL_SERVER_ERROR");
                    return;
                }
                sendPlainText(res, 200, "OK");
            });
        } else {
            sendPlainText(res, 404, "NOT_FOUND");
        }
    });

    server.listen(port, () => {
        log(`Server launched on ${port}`);
    });
}
