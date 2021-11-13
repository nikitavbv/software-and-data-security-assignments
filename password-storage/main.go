package main

import (
	"net/http"
	"password-storage/handlers"
	"password-storage/server"
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
