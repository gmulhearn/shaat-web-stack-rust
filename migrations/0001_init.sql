CREATE TABLE Users (
    id varchar(255) NOT NULL,
    username varchar(255) NOT NULL,
    password_hash varchar NOT NULL,
    PRIMARY KEY (id)
);

CREATE UNIQUE INDEX user_by_username ON Users (username);

CREATE TABLE Todos (
    id varchar(255) NOT NULL,
    user_id varchar(255) NOT NULL,
    name varchar(255) NOT NULL,
    complete boolean NOT NULL,
    PRIMARY KEY (id),
    FOREIGN KEY (user_id) REFERENCES Users(id)
);

CREATE INDEX todos_by_user_id ON Todos (user_id);
