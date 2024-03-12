package web

import "github.com/gorilla/mux"

func (app *App) Routes() *mux.Router {
	router := mux.NewRouter()

	app.Logger.Info("Registering routes")
	router.HandleFunc("/health", app.health).Methods("GET")
	router.HandleFunc("/{id}", loggerMiddleware(app.Logger, app.findProject)).Methods("GET")

	return router
}
