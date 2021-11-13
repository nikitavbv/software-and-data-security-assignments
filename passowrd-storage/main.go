package main

import (
	"net/http"
	"passowrd-storage/handlers"
	"passowrd-storage/server"
)

func runHandler() {
	http.HandleFunc("/", handlers.MainHandler)
	http.HandleFunc("/sign-up", handlers.SignUpHandler)
	http.HandleFunc("/sign-in", handlers.SignInHandler)
}

func main()  {
	runHandler()

	server.Init()
}
