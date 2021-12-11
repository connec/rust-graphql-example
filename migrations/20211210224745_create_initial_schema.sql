CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TYPE episode AS ENUM ('new_hope', 'empire', 'jedi');

CREATE TABLE humans (
	id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
	name TEXT NOT NULL UNIQUE,
	appears_in episode[] NOT NULL,
	home_planet TEXT NOT NULL
);
