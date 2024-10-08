package main

import (
	"log/slog"
	"net/http"
)

func addRoutes(
	mux *http.ServeMux,
	logger *slog.Logger,
	config *Config,
) {
	mux.Handle("/api/v1/hello", handleSomething(logger))
}
