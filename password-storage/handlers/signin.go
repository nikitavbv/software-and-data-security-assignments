package handlers

import (
	"fmt"
	"net/http"
)

func SignInHandler(w http.ResponseWriter, r *http.Request) {
	fmt.Fprintf(w, "Sing in page\n")
}
