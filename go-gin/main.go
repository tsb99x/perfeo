package main

import (
	"context"
	"log"
	"net/http"

	"github.com/gin-gonic/gin"
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

	r := gin.New()
	r.POST("/", func(c *gin.Context) {
		var doc interface{}

		err := c.BindJSON(&doc)
		if err != nil {
			c.String(http.StatusBadRequest, "BAD_REQUEST")
			log.Fatal(err)
			return
		}

		_, er := collection.InsertOne(context.Background(), doc)
		if er != nil {
			c.String(http.StatusInternalServerError, "INTERNAL_SERVER_ERROR")
			log.Fatal(er)
			return
		}

		c.String(http.StatusOK, "OK")
	})
	r.Run() // listen and serve on 0.0.0.0:8080
}

// package main

// import (
// 	"context"
// 	"net/http"
// 	"time"

// 	"github.com/gin-gonic/gin"
// 	"github.com/mongodb/mongo-go-driver/mongo"
// 	"github.com/mongodb/mongo-go-driver/mongo/readpref"
// )

// func main() {
// 	ctx, _ := context.WithTimeout(context.Background(), 10*time.Second)
// 	client, _ := mongo.Connect(ctx, "mongodb://db:27017")
// 	err := client.Ping(ctx, readpref.Primary())
// 	if err != nil {
// 		println("Failed to connect to database")
// 	}

// 	collection := client.Database("test").Collection("test")

// 	r := gin.New()
// 	r.POST("/", func(c *gin.Context) {
// 		var doc interface{}

// 		err := c.BindJSON(&doc)
// 		if err != nil {
// 			c.String(http.StatusBadRequest, "BAD_REQUEST")
// 			return
// 		}

// 		_, er := collection.InsertOne(ctx, doc)
// 		if er != nil {
// 			c.String(http.StatusInternalServerError, "INTERNAL_SERVER_ERROR")
// 			println("%v", er)
// 			return
// 		}

// 		c.String(http.StatusOK, "OK")
// 	})
// 	r.Run() // listen and serve on 0.0.0.0:8080
// }
