CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS location (
        id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
        name VARCHAR(255) NOT NULL UNIQUE
    );

CREATE TABLE IF NOT EXISTS room (
        id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
        name VARCHAR(255) NOT NULL UNIQUE,
        loc_id UUID REFERENCES location(id) ON DELETE CASCADE,
        UNIQUE (name, loc_id)
    );

CREATE TABLE IF NOT EXISTS device (
        id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
        name VARCHAR(255) NOT NULL UNIQUE,
        room_id UUID REFERENCES room(id) ON DELETE CASCADE,
        UNIQUE (name, room_id)
    );
