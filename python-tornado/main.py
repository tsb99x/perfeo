import tornado.ioloop
import tornado.web
import motor.motor_tornado
import os

class MainHandler(tornado.web.RequestHandler):
    async def post(self):
        doc = tornado.escape.json_decode(self.request.body)
        db = self.settings['db']
        await db['test'].insert_one(doc)
        self.write("OK")

if __name__ == "__main__":
    app = tornado.web.Application([
        (r"/", MainHandler),
    ])

    server = tornado.httpserver.HTTPServer(app)
    server.bind(8080)

    #server.start(0 if hasattr(os, 'fork') else 1)
    server.start(0)

    client = motor.motor_tornado.MotorClient('mongodb://db:27017')
    app.settings['db'] = client['test']
    tornado.ioloop.IOLoop.current().start()
