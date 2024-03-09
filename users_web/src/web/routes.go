package web

import (
	"github.com/gorilla/mux"
)

func (app *App) Routes() *mux.Router {
	router := mux.NewRouter()

	app.Logger.Info("Registering routes")
	router.HandleFunc("/health", app.health).Methods("GET")
	router.HandleFunc("/{username:@[0-9a-zA-Z]+}", app.findByUsername).Methods("GET")
	router.HandleFunc("/{id}", app.find).Methods("GET")

	return router
}
