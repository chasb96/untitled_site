package main

import (
	"log/slog"
	"net/http"
)

type App struct {
	logger *slog.Logger
}

func main() {
	app := App{
		logger: slog.Default(),
	}

	app.logger.Info("Binding to 0.0.0.0:80")
	host := &http.Server{
		Addr:    "0.0.0.0:80",
		Handler: app.routes(),
	}

	app.logger.Info("Serving on 0.0.0.0:80")
	if err := host.ListenAndServe(); err != nil {
		app.logger.Error("Failed to serve on 0.0.0.0:80")
	}
}
