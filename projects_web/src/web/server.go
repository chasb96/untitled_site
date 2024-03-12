package web

import (
	"log/slog"
	"projects_web/src/repository"
)

type Server struct {
	Logger   *slog.Logger
	Database *repository.Database
}

func NewApp(logger *slog.Logger, db *repository.Database) *Server {
	return &Server{
		Logger:   logger,
		Database: db,
	}
}
