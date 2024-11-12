-- Your SQL goes here
CREATE TABLE work_type (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL
);

CREATE TABLE work (
    id INTEGER PRIMARY KEY,
    title TEXT NOT NULL,
    release_date DATE,
    work_type INTEGER NOT NULL,
    summary TEXT,
    runtime INTEGER,
    language TEXT,
    network TEXT,
    status TEXT,
    FOREIGN KEY (work_type) REFERENCES work_type(id)
);

CREATE TABLE genre (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL
);

CREATE TABLE work_genre (
    work_id INTEGR NOT NULL,
    genre_id INTEGER NOT NULL,
    FOREIGN KEY (work_id) REFERENCES work(id),
    FOREIGN KEY (genre_id) REFERENCES genre(id)
);

CREATE TABLE person (
    id INTEGER PRIMARY KEY,
    prefix TEXT,
    first_name TEXT NOT NULL,
    middle_names TEXT,         
    last_name TEXT NOT NULL,    
    suffix TEXT,
    date_of_birth DATE,
    date_of_death DATE,
    biography TEXT,
    nationality TEXT
);

CREATE TABLE role (
    id INTEGER PRIMARY KEY,
    role_name TEXT NOT NULL
);

CREATE TABLE work_person (
    id INTEGER PRIMARY KEY,
    work_id INTEGER NOT NULL,
    person_id INTEGER NOT NULL,
    role_id INTEGER NOT NULL,
    character_suffix TEXT,
    character_first_name TEXT NOT NULL,
    character_middle_names TEXT,         
    character_last_name TEXT NOT NULL,    
    character_prefix TEXT,
    FOREIGN KEY (work_id) REFERENCES work(id),
    FOREIGN KEY (person_id) REFERENCES person(id),
    FOREIGN KEY (role_id) REFERENCES role(id)
);

CREATE TABLE review (
    id INTEGER PRIMARY KEY,
    work_id INTEGER NOT NULL,
    review_text TEXT,
    reviewer_name TEXT,
    review_date DATE,
    FOREIGN KEY (work_id) REFERENCES work(id)
);

CREATE TABLE rating (
    id INTEGER PRIMARY KEY,
    work_id INTEGER NOT NULL,
    rating_value INTEGER NOT NULL CHECK (rating_value BETWEEN 1 AND 10),
    rating_source TEXT,
    rating_date DATE,
    FOREIGN KEY (work_id) REFERENCES work(id)
);

CREATE TABLE artwork (
    id INTEGER PRIMARY KEY,
    work_id INTEGER NOT NULL,
    file_path TEXT NOT NULL,   
    image_type INTEGER not null,
    image_description TEXT,
    FOREIGN KEY (work_id) REFERENCES work(id)
    FOREIGN KEY (image_type) REFERENCES artwork_type(id)
);

CREATE TABLE artwork_type (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL
);


CREATE VIEW work_with_details AS
SELECT 
    work.id AS work_id,
    work.title,
    work.release_date,
    work.work_type,    
    work.summary,
    work.runtime,
    work.language,
    work.network,
    work.status,
    work_type.name AS work_type_name,
    rating.id AS rating_id,
    rating.rating_value,
    rating.rating_source,
    rating.rating_date
FROM 
    work
LEFT JOIN 
    work_type ON work.work_type = work_type.id
LEFT JOIN 
    rating ON work.id = rating.work_id;
