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

type CreateProjectRequest struct {
	UserId      int
	ProjectName string
}

func (db *Database) CreateProject(request *CreateProjectRequest) (int, error) {
	const CREATE_PROJECT_QUERY = `
		INSERT INTO projects
			(name, user_id)
		VALUES
			($1, $2)
		RETURNING
			id
	`

	var id int

	err := db.pool.
		QueryRow(context.Background(), CREATE_PROJECT_QUERY, request.ProjectName, request.UserId).
		Scan(&id)

	if err != nil {
		return 0, err
	}

	return id, nil
}

func (db *Database) ListByUser(userId int) ([]Project, error) {
	const LIST_BY_USER_QUERY = `
		SELECT
			id,
			name
		FROM
			projects
		WHERE
			user_id = $1
	`

	var projects []Project

	rows, err := db.pool.Query(context.Background(), LIST_BY_USER_QUERY, userId)
	if err != nil {
		return nil, err
	}

	for rows.Next() {
		var id int
		var name string

		err := rows.Scan(&id, &name)
		if err != nil {
			return nil, err
		}

		project := Project{
			Id:   id,
			Name: name,
		}

		projects = append(projects, project)
	}

	return projects, nil
}
