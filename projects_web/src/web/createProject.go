package web

import (
	"encoding/json"
	"net/http"
	"projects_web/src/repository"
)

type CreateProjectRequest struct {
	Name string `json:"name"`
}

type CreateProjectResponse struct {
	Id int `json:"id"`
}

func (app *App) createProject(w http.ResponseWriter, r *http.Request, user *AuthenticatedUser) {
	var requestBody CreateProjectRequest
	err := json.NewDecoder(r.Body).Decode(&requestBody)
	if err != nil {
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	createProjectRequest := repository.CreateProjectRequest{
		UserId:      user.Id,
		ProjectName: requestBody.Name,
	}

	projectId, err := app.Database.CreateProject(&createProjectRequest)
	if err != nil {
		app.Logger.Error(err.Error())
		w.WriteHeader(http.StatusInternalServerError)
		return
	}

	responseBody := CreateProjectResponse{
		Id: projectId,
	}

	responseBodySerialized, err := json.Marshal(responseBody)
	if err != nil {
		app.Logger.Error(err.Error())
		w.WriteHeader(http.StatusInternalServerError)
		return
	}

	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(http.StatusOK)
	w.Write(responseBodySerialized)
}
