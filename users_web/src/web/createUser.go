package web

import (
	"io"
	"net/http"
)

type CreateUserRequest struct {
	Username string `json:"username"`
	Password string `json:"password"`
}

type CreateUserResponse struct {
	Id int `json:"id"`
}

func (app *App) createUser(w http.ResponseWriter, r *http.Request) {
	response, err := http.Post("http://auth/create_user", "application/json", r.Body)
	if err != nil {
		app.Logger.Error(err.Error())
		w.WriteHeader(http.StatusInternalServerError)
		return
	}

	responseBody, err := io.ReadAll(response.Body)
	if err != nil {
		app.Logger.Error(err.Error())
		w.WriteHeader(http.StatusInternalServerError)
		return
	}

	w.WriteHeader(response.StatusCode)
	w.Header().Set("Content-Type", "application/json")
	w.Write(responseBody)
}
