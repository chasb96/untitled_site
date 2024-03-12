package repository

import (
	"context"
	"os"
	"strings"

	"github.com/jackc/pgx/v5/pgxpool"
)

type Database struct {
	pool *pgxpool.Pool
}

func NewDatabase() *Database {
	connection_string := strings.Trim(os.Getenv("DATABASE_URL"), "\n")

	pool, err := pgxpool.New(context.Background(), connection_string)
	if err != nil {
		panic(err)
	}

	return &Database{
		pool: pool,
	}
}

func (conn *Database) Close() {
	conn.pool.Close()
}
