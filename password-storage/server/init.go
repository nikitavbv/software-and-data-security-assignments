package server

import (
	"log"
	"net/http"
)

func Init()  {
	log.Fatal(http.ListenAndServe(":8080", nil))
}