package main

import (
	"database/sql"
	"flag"
	"log"
	"net/http"
	"os"

	"github.com/chaoticenginerd/playground/snipstore/pkg/models/sqlite"
	_ "modernc.org/sqlite"
)

type application struct {
	errorLog *log.Logger
	infoLog  *log.Logger
	snippets *sqlite.SnippetModel
}

func main() {
	addr := flag.String("addr", "4000", "HTTP network address")
	dsn := flag.String("dsn", "snipstore.db", "Sqlite data source name.")
	flag.Parse()

	errorLog := log.New(os.Stderr, "ERROR\t", log.Ldate|log.Ltime|log.Lshortfile)
	infoLog := log.New(os.Stdout, "INFO\t", log.Ldate|log.Ltime|log.LUTC)

	db, err := openDB(*dsn)
	if err != nil {
		errorLog.Fatal(err)
	}
	defer db.Close()

	app := &application{
		errorLog: errorLog,
		infoLog:  infoLog,
		snippets: &sqlite.SnippetModel{DB: db},
	}

	srv := &http.Server{
		Addr:     ":" + *addr,
		ErrorLog: errorLog,
		Handler:  app.routes(),
	}

	infoLog.Printf("Starting server on port %s", srv.Addr)
	err = srv.ListenAndServe()
	errorLog.Fatal(err)
}

func openDB(dsn string) (*sql.DB, error) {
	db, err := sql.Open("sqlite", dsn)
	if err != nil {
		return nil, err
	}

	if err = db.Ping(); err != nil {
		return nil, err
	}
	return db, nil
}
