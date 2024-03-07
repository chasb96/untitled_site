package main

import (
	"log/slog"
	"net/http"

	"github.com/chasb96/untitled_site/user_service/cmd/repository"
)

type App struct {
	logger *slog.Logger
	db     *repository.DatabaseConnection
}

func main() {
	dbPool := repository.NewDatabaseConnection()
	defer dbPool.Close()

	app := App{
		logger: slog.Default(),
		db:     dbPool,
	}

	app.logger.Info("Binding to 0.0.0.0:80")
	host := &http.Server{
		Addr:    "0.0.0.0:80",
		Handler: app.routes(),
	}

	app.logger.Info("Serving on 0.0.0.0:80")
	if err := host.ListenAndServe(); err != nil {
		app.logger.Error("Failed to serve on 0.0.0.0:80")
		app.logger.Error(err.Error())
	}
}
