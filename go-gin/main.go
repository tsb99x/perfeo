package main

import (
	"context"
	"log"
	"net/http"
	"os"

	"github.com/gin-gonic/gin"
	"github.com/mongodb/mongo-go-driver/mongo"
)

func getEnvVar(varName string) string {
	res := os.Getenv(varName)
	if len(res) == 0 {
		log.Fatal("Failed to get '" + varName + "' env var")
	}
	return res
}

func main() {
	dbURI := getEnvVar("DB_URI")
	port := getEnvVar("PORT")

	client, err := mongo.Connect(context.TODO(), dbURI)
	if err != nil {
		log.Fatal(err)
	}

	err = client.Ping(context.TODO(), nil)
	if err != nil {
		log.Fatal(err)
	}

	collection := client.Database("test").Collection("test")

	r := gin.New()

	r.GET("/ping", func(c *gin.Context) {
		c.String(http.StatusOK, "PONG")
	})

	r.POST("/", func(c *gin.Context) {
		var doc interface{}

		err := c.BindJSON(&doc)
		if err != nil {
			c.String(http.StatusBadRequest, "BAD_REQUEST")
			return
		}

		_, err = collection.InsertOne(context.Background(), doc)
		if err != nil {
			c.String(http.StatusInternalServerError, "INTERNAL_SERVER_ERROR")
			return
		}

		c.String(http.StatusOK, "OK")
	})

	r.Run("0.0.0.0:" + port)
}
