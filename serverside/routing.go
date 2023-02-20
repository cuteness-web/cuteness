package main

import (
	"log"
	"fmt"
	"net/http"
)

func main() {
    fmt.Printf("Starting server at port 8080\n")
    http.HandleFunc("/hello", func(w http.ResponseWriter, r *http.Request) {helloHandler(w, r)})
	if err := http.ListenAndServe(":8080", nil); err != nil {
        log.Fatal(err)
    }
}

func helloHandler(w http.ResponseWriter, r *http.Request) {
    if r.URL.Path != "/hello" {
        http.Error(w, "404 not found.", http.StatusNotFound)
        return
    }

    if r.Method != "GET" {
        http.Error(w, "Method is not supported.", http.StatusNotFound)
        return
    }


    fmt.Fprintf(w, "Hello!")
}