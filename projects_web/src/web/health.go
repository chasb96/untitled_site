package web

import (
	"net/http"
)

func (*Server) health(w http.ResponseWriter, r *http.Request) {
	w.WriteHeader(http.StatusOK)
}
