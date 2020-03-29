package perfeo

import io.vertx.core.AbstractVerticle
import io.vertx.core.Future
import io.vertx.core.json.JsonObject
import io.vertx.ext.mongo.MongoClient
import io.vertx.ext.web.Router
import io.vertx.ext.web.handler.BodyHandler

class MainVerticle : AbstractVerticle() {

    override fun start(startFuture: Future<Void>) {
        val router = Router.router(vertx)

        val port = System.getenv("PORT").toInt()
        val uri = System.getenv("DB_URI")

        val dbConfig = JsonObject()
            .put("connection_string", uri)
            .put("db_name", "test")

        val client = MongoClient.createShared(vertx, dbConfig)

        router.route().handler(BodyHandler.create())

        router.get("/ping").handler { rc ->
            rc.response()
                .putHeader("Content-Type", "text/plain")
                .setStatusCode(200)
                .end("PONG")
        }

        router.post("/").handler { rc ->
            rc.bodyAsJson?.let { json ->
                client.save("test", json) { rs ->
                    if (rs.succeeded()) {
                        rc.response()
                            .putHeader("Content-Type", "text/plain")
                            .setStatusCode(200)
                            .end("OK")
                    } else {
                        rc.response()
                            .putHeader("Content-Type", "text/plain")
                            .setStatusCode(500)
                            .end("INTERNAL_SERVER_ERROR")
                        throw rs.cause()
                    }
                }
            } ?: run {
                rc.response()
                    .putHeader("Content-Type", "text/plain")
                    .setStatusCode(400)
                    .end("BAD_REQUEST")
            }
        }

        vertx.createHttpServer()
            .requestHandler(router::handle)
            .listen(port) { http ->
                if (http.succeeded()) {
                    startFuture.complete()
                    println("HTTP server started on port 8080")
                } else {
                    startFuture.fail(http.cause())
                }
            }
    }
}
