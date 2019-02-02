package main

import (
	"context"
	"log"

	"github.com/kataras/iris"
	"github.com/mongodb/mongo-go-driver/mongo"
)

func main() {
	client, err := mongo.Connect(context.TODO(), "mongodb://db:27017")
	if err != nil {
		log.Fatal(err)
	}

	err2 := client.Ping(context.TODO(), nil)
	if err2 != nil {
		log.Fatal(err2)
	}

	collection := client.Database("test").Collection("test")

	app := iris.New()
	app.Post("/", func(c iris.Context) {
		var doc interface{}

		err := c.ReadJSON(&doc)
		if err != nil {
			c.StatusCode(iris.StatusBadRequest)
			c.WriteString("BAD_REQUEST")
			return
		}

		_, er := collection.InsertOne(context.Background(), doc)
		if er != nil {
			c.StatusCode(iris.StatusInternalServerError)
			c.WriteString("INTERNAL_SERVER_ERROR")
			return
		}

		c.StatusCode(iris.StatusOK)
		c.WriteString("OK")
	})
	// listen and serve on http://0.0.0.0:8080.
	app.Run(iris.Addr(":8080"))
}
