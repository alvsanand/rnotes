CREATE SCHEMA rnotes;
SET
  search_path TO rnotes;
CREATE SEQUENCE users_id_seq;
CREATE TABLE users (
    id INTEGER NOT NULL DEFAULT nextval('users_id_seq'),
    email VARCHAR(256) NOT NULL,
    name VARCHAR(256) NOT NULL,
    password VARCHAR(256) NOT NULL,
    create_time TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    update_time TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (id),
    CONSTRAINT unq_users UNIQUE(name)
  );
ALTER SEQUENCE users_id_seq OWNED BY users.id;
CREATE SEQUENCE categories_id_seq;
CREATE TABLE categories (
    id INTEGER NOT NULL DEFAULT nextval('categories_id_seq'),
    name VARCHAR(256) NOT NULL,
    create_time TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    update_time TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (id),
    CONSTRAINT unq_categories UNIQUE(name)
  );
ALTER SEQUENCE categories_id_seq OWNED BY categories.id;
CREATE SEQUENCE notes_id_seq;
CREATE TABLE notes (
    id INTEGER NOT NULL DEFAULT nextval('notes_id_seq'),
    user_id INTEGER NOT NULL,
    category_id INTEGER DEFAULT NULL,
    title VARCHAR(256) DEFAULT NULL,
    data TEXT,
    create_time TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    update_time TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (id),
    FOREIGN KEY (category_id) REFERENCES categories (id),
    FOREIGN KEY (user_id) REFERENCES users (id)
  );
ALTER SEQUENCE notes_id_seq OWNED BY categories.id;