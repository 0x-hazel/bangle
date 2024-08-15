CREATE TABLE lists (
    id serial PRIMARY KEY,
    fallback text NOT NULL DEFAULT '',
    edit_pw text NOT NULL,
    read_pw text NOT NULL
);

CREATE TABLE bangs (
    list_id integer REFERENCES lists (id),
    bang text,
    link text NOT NULL,
    PRIMARY KEY (list_id, bang)
);