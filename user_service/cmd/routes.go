package main

import (
	"github.com/gorilla/mux"
)

func (app *App) routes() *mux.Router {
	router := mux.NewRouter()

	app.logger.Info("Registering routes")
	router.HandleFunc("/health", app.health).Methods("GET")

	return router
}
