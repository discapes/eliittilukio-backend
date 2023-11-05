CREATE TABLE feedback (
	username TEXT NOT NULL,
	email TEXT NOT NULL,
	text TEXT NOT NULL,
	created TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE TABLE users (
	username TEXT NOT NULL,
	email TEXT NOT NULL,
	created TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
	modified TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
	banned INT NOT NULL DEFAULT 0,
	score INT NOT NULL DEFAULT 0,
	hash TEXT NOT NULL,
	PRIMARY KEY (email),
	UNIQUE (username)
);