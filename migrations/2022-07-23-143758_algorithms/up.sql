-- Your SQL goes here
(
    id         SERIAL PRIMARY KEY,
    algoname   VARCHAR NOT NULL,
    description   VARCHAR NOT NULL,
    properties VARCHAR NOT NULL,
    time_complexity VARCHAR NOT NULL,
    space_complexity VARCHAR NOT NULL,
)