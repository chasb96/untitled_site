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

	server := web.Server{
		Logger:   slog.Default(),
		Database: db,
	}

	server.Logger.Info("Binding to " + bind)
	host := &http.Server{
		Addr:    bind,
		Handler: server.Routes(),
	}

	server.Logger.Info("Serving on " + bind)
	if err := host.ListenAndServe(); err != nil {
		server.Logger.Error("Failed to serve on " + bind)
		server.Logger.Error(err.Error())
	}
}
