package web

import (
	"log/slog"
	"projects_web/src/repository"
)

type App struct {
	Logger   *slog.Logger
	Database *repository.Database
}

func NewApp(logger *slog.Logger, db *repository.Database) *App {
	return &App{
		Logger:   logger,
		Database: db,
	}
}
