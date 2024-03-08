package repository

import (
	"context"
	"os"
	"strings"

	"github.com/jackc/pgx/v5/pgxpool"
)

type DatabaseConnection struct {
	inner *pgxpool.Pool
}

func NewDatabaseConnection() *DatabaseConnection {
	connection_string := strings.Trim(os.Getenv("DATABASE_URL"), "\n")

	pool, err := pgxpool.New(context.Background(), connection_string)
	if err != nil {
		panic(err)
	}

	return &DatabaseConnection{
		inner: pool,
	}
}

func (conn *DatabaseConnection) Close() {
	conn.inner.Close()
}
