package repository

import (
	"context"

	"github.com/jackc/pgx/v5"
)

func (db *DatabaseConnection) Find(id int) (*User, error) {
	const FIND_USER_QUERY = `
		SELECT
			username
		FROM
			users
		WHERE
			id = $1
	`

	var username string

	err := db.inner.QueryRow(context.Background(), FIND_USER_QUERY, id).Scan(&username)
	if err != nil {
		if err == pgx.ErrNoRows {
			return nil, nil
		}

		return nil, err
	}

	user := User{
		Id:       id,
		Username: username,
	}

	return &user, nil
}

func (db *DatabaseConnection) FindByUsername(username string) (*User, error) {
	const FIND_USER_QUERY = `
		SELECT
			id
		FROM
			users
		WHERE
			username = $1
	`

	var id int

	err := db.inner.QueryRow(context.Background(), FIND_USER_QUERY, username).Scan(&id)
	if err != nil {
		if err == pgx.ErrNoRows {
			return nil, nil
		}

		return nil, err
	}

	user := User{
		Id:       id,
		Username: username,
	}

	return &user, nil
}
