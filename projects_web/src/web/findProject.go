package web

import (
	"encoding/json"
	"net/http"
	"strconv"

	"github.com/gorilla/mux"
)

type FindProjectResponse struct {
	Id   int    `json:"id"`
	Name string `json:"name"`
}

func (app *App) findProject(w http.ResponseWriter, r *http.Request) {
	vars := mux.Vars(r)

	id, err := strconv.Atoi(vars["id"])
	if err != nil {
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	project, err := app.Database.FindProject(id)
	if err != nil {
		app.Logger.Error(err.Error())
		w.WriteHeader(http.StatusInternalServerError)
		return
	}

	if project == nil {
		w.WriteHeader(http.StatusNotFound)
		return
	}

	body := FindProjectResponse{
		Id:   project.Id,
		Name: project.Name,
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
