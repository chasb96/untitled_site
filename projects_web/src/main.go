package main

import (
	"flag"
	"log/slog"
	"net/http"
	"projects_web/src/repository"
	"projects_web/src/web"
	"strconv"
)

func main() {
	port := flag.Int("port", 80, "Bind to port")
	address := flag.String("address", "0.0.0.0", "Serve on address")

	bind := (*address) + ":" + strconv.Itoa(*port)

	db := repository.NewDatabase()
	defer db.Close()

	app := web.App{
		Logger:   slog.Default(),
		Database: db,
	}

	app.Logger.Info("Binding to " + bind)
	host := &http.Server{
		Addr:    bind,
		Handler: app.Routes(),
	}

	app.Logger.Info("Serving on " + bind)
	if err := host.ListenAndServe(); err != nil {
		app.Logger.Error("Failed to serve on " + bind)
		app.Logger.Error(err.Error())
	}
}
