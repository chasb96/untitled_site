package web

import (
	"bytes"
	"encoding/json"
	"log/slog"
	"net/http"
	"strings"
)

type AuthenticateRequest struct {
	Token string `json:"token"`
}

type AuthenticatedUser struct {
	Id int `json:"id"`
}

func authMiddleware(
	logger *slog.Logger,
	handler func(w http.ResponseWriter, r *http.Request, u *AuthenticatedUser),
) func(w http.ResponseWriter, r *http.Request) {
	return func(w http.ResponseWriter, r *http.Request) {
		authHeader := r.Header.Get("Authorization")

		if !strings.HasPrefix(authHeader, "Bearer") && len(authHeader) > 7 {
			w.WriteHeader(http.StatusBadRequest)
			return
		}

		requestBody, err := json.Marshal(AuthenticateRequest{
			Token: authHeader[7:],
		})
		if err != nil {
			w.WriteHeader(http.StatusInternalServerError)
			return
		}

		authenticateResponse, err := http.Post("http://auth/authenticate", "application/json", bytes.NewBuffer(requestBody))
		if err != nil {
			w.WriteHeader(http.StatusUnauthorized)
			return
		}

		switch statusCode := authenticateResponse.StatusCode; statusCode {
		case 500:
			w.WriteHeader(http.StatusInternalServerError)
			return
		case 401:
			w.WriteHeader(http.StatusUnauthorized)
			return
		}

		authenticatedUser := AuthenticatedUser{}
		json.NewDecoder(authenticateResponse.Body).Decode(&authenticatedUser)

		handler(w, r, &authenticatedUser)
	}
}
