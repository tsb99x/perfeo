import asyncio
import uvloop

from signal import signal, SIGINT

asyncio.set_event_loop_policy(uvloop.EventLoopPolicy())
import os

from motor.motor_asyncio import AsyncIOMotorClient
from sanic import Sanic
from sanic import response

app = Sanic(__name__)

@app.listener('before_server_start')
def init(sanic, loop):
    global db
    db = AsyncIOMotorClient('mongodb://db:27017').test

@app.route("/", methods=["POST"])
async def new(request):
    contact = request.json
    await db.test.insert_one(contact)
    return response.text('OK')

if __name__ == '__main__':
    # asyncio.set_event_loop(uvloop.new_event_loop())
    # server = app.create_server(host="0.0.0.0", port=8080, debug=False, access_log=False)
    # loop = asyncio.get_event_loop()
    # task = asyncio.ensure_future(server)
    # signal(SIGINT, lambda s, f: loop.stop())
    # try:
    #     loop.run_forever()
    # except:
    #     loop.stop()
    app.run(host="0.0.0.0", port=8080, workers=os.cpu_count(), debug=False, access_log=False)
