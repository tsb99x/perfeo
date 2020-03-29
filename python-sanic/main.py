import os

from motor.motor_asyncio import AsyncIOMotorClient
from sanic import Sanic
from sanic import response

app = Sanic(__name__)

@app.listener('before_server_start')
def init(sanic, loop):
    global db
    db = AsyncIOMotorClient('mongodb://db:27017').test

@app.route("/ping", methods=["GET"])
async def new(request):
    return response.text('PONG')

@app.route("/", methods=["POST"])
async def new(request):
    contact = request.json
    await db.test.insert_one(contact)
    return response.text('OK')

if __name__ == '__main__':
    app.run(host="0.0.0.0", port=8080, workers=os.cpu_count(), debug=False, access_log=False)
