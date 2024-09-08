CREATE TABLE games (
  id integer NOT NULL PRIMARY KEY,
  name text NOT NULL,
  caption varchar,
  developer integer,
  image text,
  direcroty text,
  executive text,
  installed datetime NOT NULL,
  playtime integer NOT NULL
);

CREATE INDEX index_1 ON games (id);
CREATE INDEX index_4 ON games (installed);

CREATE TABLE tags (
  id integer NOT NULL PRIMARY KEY,
  name text NOT NULL
);

CREATE INDEX index_1 ON tags (id);

CREATE TABLE game_tags (
  game_id integer NOT NULL PRIMARY KEY,
  tag_id integer NOT NULL PRIMARY KEY
);


CREATE TABLE genres (
  id integer NOT NULL PRIMARY KEY,
  name text NOT NULL
);


CREATE TABLE game_genres (
  game_id integer NOT NULL PRIMARY KEY,
  genre_id integer NOT NULL PRIMARY KEY
);


CREATE TABLE sessions (
  id integer NOT NULL PRIMARY KEY,
  game_id integer NOT NULL,
  start_time datetime NOT NULL,
  end_time datetime,
  duration integer
);


CREATE TABLE developers (
  id integer NOT NULL PRIMARY KEY,
  name text NOT NULL
);


ALTER TABLE games ADD CONSTRAINT games_id_fk FOREIGN KEY (id) REFERENCES game_tags (game_id);
ALTER TABLE game_tags ADD CONSTRAINT game_tags_tag_id_fk FOREIGN KEY (tag_id) REFERENCES tags (id);
ALTER TABLE games ADD CONSTRAINT games_id_fk FOREIGN KEY (id) REFERENCES game_genres (game_id);
ALTER TABLE game_genres ADD CONSTRAINT game_genres_genre_id_fk FOREIGN KEY (genre_id) REFERENCES genres (id);
ALTER TABLE games ADD CONSTRAINT games_id_fk FOREIGN KEY (id) REFERENCES sessions (game_id);
ALTER TABLE games ADD CONSTRAINT games_developer_fk FOREIGN KEY (developer) REFERENCES developers (id);
