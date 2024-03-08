package main

import (
	"log/slog"
	"net/http"

	"users_web/src/repository"
	"users_web/src/web"
)

func main() {
	dbPool := repository.NewDatabaseConnection()
	defer dbPool.Close()

	app := web.App{
		Logger:   slog.Default(),
		Database: dbPool,
	}

	app.Logger.Info("Binding to 0.0.0.0:80")
	host := &http.Server{
		Addr:    "0.0.0.0:8080",
		Handler: app.Routes(),
	}

	app.Logger.Info("Serving on 0.0.0.0:80")
	if err := host.ListenAndServe(); err != nil {
		app.Logger.Error("Failed to serve on 0.0.0.0:80")
		app.Logger.Error(err.Error())
	}
}
