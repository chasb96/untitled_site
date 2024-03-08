package web

import (
	"encoding/json"
	"net/http"

	"github.com/gorilla/mux"
)

type FindUsernameResponse struct {
	Id       int    `json:"id"`
	Username string `json:"username"`
}

func (app *App) findByUsername(w http.ResponseWriter, r *http.Request) {
	vars := mux.Vars(r)

	username := vars["username"][1:]

	user, err := app.Database.FindByUsername(username)
	if err != nil {
		app.Logger.Error(err.Error())
		w.WriteHeader(http.StatusInternalServerError)
		return
	}

	if user == nil {
		w.WriteHeader(http.StatusNotFound)
		return
	}

	body := FindUserResponse{
		Id:       user.Id,
		Username: user.Username,
	}

	body_stringified, err := json.Marshal(body)
	if err != nil {
		app.Logger.Error(err.Error())
		w.WriteHeader(http.StatusInternalServerError)
		return
	}

	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(http.StatusOK)
	w.Write(body_stringified)
}
