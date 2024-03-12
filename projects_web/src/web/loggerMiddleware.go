package web

import (
	"log/slog"
	"net/http"
)

func loggerMiddleware(logger *slog.Logger, handler func(w http.ResponseWriter, r *http.Request)) func(w http.ResponseWriter, r *http.Request) {
	return func(w http.ResponseWriter, r *http.Request) {
		logger.Info(r.Method + " " + r.RequestURI)

		handler(w, r)
	}
}
