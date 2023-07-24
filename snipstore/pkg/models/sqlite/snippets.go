package sqlite

import (
	"database/sql"
	"errors"

	"github.com/chaoticenginerd/playground/snipstore/pkg/models"
)

type SnippetModel struct {
	DB *sql.DB
}

func (sm *SnippetModel) Insert(title, content, expires string) (int, error) {

	query := `INSERT INTO snippets(title, content, created_at, expires_at)
    VALUES (?, ?, DATETIME('now'), DATETIME('now', ?))`

	result, err := sm.DB.Exec(query, title, content, expires)
	if err != nil {
		return 0, err
	}

	id, err := result.LastInsertId()
	if err != nil {
		return 0, err
	}
	return int(id), nil
}

func (sm *SnippetModel) Get(id int) (*models.Snippet, error) {
	query := `SELECT id, title, content, created_at, expires_at
    FROM snippets
    WHERE expires_at > DATETIME('now') AND id = ?`
	row := sm.DB.QueryRow(query, id)
	snippet := &models.Snippet{}
	err := row.Scan(&snippet.ID, &snippet.Title, &snippet.Content, &snippet.CreatedAt, &snippet.ExpiresAt)
	if err != nil {
		if errors.Is(err, sql.ErrNoRows) {
			return nil, models.ErrNoRecord
		} else {
			return nil, err
		}
	}
	return snippet, nil
}

func (sm *SnippetModel) Latest() ([]*models.Snippet, error) {
	query := `SELECT id, title, content, created_at, expires_at
    FROM snippets
    WHERE expires_at > DATETIME('now')
    ORDER BY created_at DESC
    LIMIT 10`

	rows, err := sm.DB.Query(query)
	if err != nil {
		return nil, err
	}
	defer rows.Close()
	snippets := []*models.Snippet{}

	for rows.Next() {
		snippet := &models.Snippet{}
		err := rows.Scan(&snippet.ID, &snippet.Title, &snippet.Content, &snippet.CreatedAt, &snippet.ExpiresAt)
		if err != nil {
			return nil, err
		}
		snippets = append(snippets, snippet)
	}

	if err = rows.Err(); err != nil {
		return nil, err
	}
	return snippets, nil
}
