package handlers

import (
	"fmt"
	"io/ioutil"
	"net/http"
)

func SignUpHandler(w http.ResponseWriter, r *http.Request) {
	if r.Method == "POST" {

		w.Header().Set("Content-Type", "application/json")

		body, err := ioutil.ReadAll(r.Body)

		if err != nil {
			http.Error(w, "Error reading request body", http.StatusInternalServerError)
		}

		data := fmt.Sprintf(`%s`, string(body))

		fmt.Fprint(w, data)
	} else {
		http.Error(w, "Invalid request method", http.StatusMethodNotAllowed)
	}
}
