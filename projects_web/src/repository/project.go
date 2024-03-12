package repository

import (
	"context"

	"github.com/jackc/pgx/v5"
)

type Project struct {
	Id   int
	Name string
}

func (db *Database) FindProject(id int) (*Project, error) {
	const FIND_PROJECT_QUERY = `
		SELECT
			name
		FROM
			projects
		WHERE
			id = $1
	`

	var name string

	err := db.pool.
		QueryRow(context.Background(), FIND_PROJECT_QUERY, id).
		Scan(&name)

	if err != nil {
		if err == pgx.ErrNoRows {
			return nil, nil
		}

		return nil, err
	}

	project := Project{
		Id:   id,
		Name: name,
	}

	return &project, nil
}
