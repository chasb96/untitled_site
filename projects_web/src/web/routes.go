package web

import "github.com/gorilla/mux"

func (server *Server) Routes() *mux.Router {
	router := mux.NewRouter()

	server.Logger.Info("Registering routes")
	router.HandleFunc("/health", server.health).Methods("GET")
	router.HandleFunc("/users/{user_id}", loggerMiddleware(server.Logger, server.listByUser)).Methods("GET")
	router.HandleFunc("/{id}", loggerMiddleware(server.Logger, server.findProject)).Methods("GET")
	router.HandleFunc("/", loggerMiddleware(server.Logger, authMiddleware(server.Logger, server.createProject))).Methods("POST")

	return router
}
