-- This file should undo anything in `up.sql`
-- drop all tables added
PRAGMA foreign_keys = ON;
PRAGMA foreign_key_check;
DROP TABLE work;
DROP TABLE work_type;
DROP TABLE genre;
DROP TABLE work_genre;
DROP TABLE person;
DROP TABLE role;
DROP TABLE work_person;
DROP TABLE review;
DROP TABLE rating;
DROP TABLE artwork;
DROP TABLE artwork_type;

