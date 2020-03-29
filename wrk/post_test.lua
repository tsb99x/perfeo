-- Very simple POST request with JSON payload, body size of 52 bytes
wrk.method = "POST"
wrk.body   = '{"some":1,"random":2,"json":3,"object":4,"struct":5}'
wrk.headers["Content-Type"] = "application/json"
