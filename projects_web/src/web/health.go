package web

import (
	"net/http"
)

func (app *App) health(w http.ResponseWriter, r *http.Request) {
	w.WriteHeader(http.StatusOK)
}
