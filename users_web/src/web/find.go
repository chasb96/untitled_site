package web

import (
	"encoding/json"
	"net/http"
	"strconv"

	"github.com/gorilla/mux"
)

type FindUserResponse struct {
	Id       int    `json:"id"`
	Username string `json:"username"`
}

func (app *App) find(w http.ResponseWriter, r *http.Request) {
	vars := mux.Vars(r)

	id, err := strconv.Atoi(vars["id"])
	if err != nil {
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	user, err := app.Database.Find(id)
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
