package web

import (
	"encoding/json"
	"net/http"
	"strconv"

	"github.com/gorilla/mux"
)

type ListByUserResponse struct {
	Projects []ListByUserResponseItem `json:"projects"`
}

type ListByUserResponseItem struct {
	Id   int    `json:"id"`
	Name string `json:"name"`
}

func (server *Server) listByUser(w http.ResponseWriter, r *http.Request) {
	vars := mux.Vars(r)

	userId, err := strconv.Atoi(vars["user_id"])
	if err != nil {
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	projects, err := server.Database.ListByUser(userId)
	if err != nil {
		server.Logger.Error(err.Error())
		w.WriteHeader(http.StatusInternalServerError)
		return
	}

	responseProjects := make([]ListByUserResponseItem, 0)
	for i := 0; i < len(projects); i++ {
		project := ListByUserResponseItem{
			Id:   projects[i].Id,
			Name: projects[i].Name,
		}

		responseProjects = append(responseProjects, project)
	}

	response := ListByUserResponse{
		Projects: responseProjects,
	}

	body, err := json.Marshal(response)
	if err != nil {
		server.Logger.Error(err.Error())
		w.WriteHeader(http.StatusInternalServerError)
		return
	}

	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(http.StatusOK)
	w.Write(body)
}
