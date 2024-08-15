package main

import (
	"log/slog"
	"net/http"
)

func handleSomething(logger *slog.Logger) http.Handler {
	type request struct {
		Name string
	}

	type response struct {
		Greeting string `json:"greeting"`
	}

	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		logger.InfoContext(r.Context(), "handling request")
		response := response{
			Greeting: "Hello world!",
		}
		encode(w, r, http.StatusOK, response)
	})
}
