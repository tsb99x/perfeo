from motor.motor_asyncio import AsyncIOMotorClient
from sanic import Sanic
from sanic.response import text

app = Sanic("perfeo")


@app.before_server_start
async def attach_db(app):
    app.ctx.db = AsyncIOMotorClient("mongodb://db:27017").test


@app.route("/ping", methods=["GET"])
async def ping(request):
    return text("PONG")


@app.route("/", methods=["POST"])
async def new(request):
    contact = request.json
    db = app.ctx.db
    await db.test.insert_one(contact)
    return text("OK")
