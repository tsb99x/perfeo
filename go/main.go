package main

import (
    "log"
    "net/http"
    "github.com/gorilla/mux"
    "github.com/mongodb/mongo-go-driver/mongo"
)

//type App struct {
//    Router *mux.Router
//    DB     *sql.DB
//}

func PostDocument(w http.ResponseWriter, r *http.Request) {
    fmt.Fprintln(w, "not implemented yet !")
}

func main() {
    client, err := mongo.NewClient("mongodb://localhost:27017")
    ctx, _ := context.WithTimeout(context.Background(), 10*time.Second)
    err = client.Connect(ctx)
    collection := client.Database("test").Collection("test")
    ctx, _ = context.WithTimeout(context.Background(), 5*time.Second)
    res, err := collection.InsertOne(ctx, bson.M{"name": "pi", "value": 3.14159})
    id := res.InsertedID

    json.Unmarshal...

    r := mux.NewRouter()
    r.HandleFunc("/", PostDocument).Methods("POST")
    if err := http.ListenAndServe(":8000", r); err != nil {
    		log.Fatal(err)
    }
}
