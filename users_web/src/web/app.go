package web

import (
	"log/slog"
	"users_web/src/repository"
)

type App struct {
	Logger   *slog.Logger
	Database *repository.DatabaseConnection
}
