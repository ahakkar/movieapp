-- Your SQL goes here

INSERT INTO work_type (name) VALUES
    ('Movie'),
    ('TV'),
    ('Short'),
    ('Special'),
    ('Miniseries'),
    ('Documentary'),
    ('TV Movie'),
    ('TV Special'),
    ('TV Miniseries'),
    ('TV Documentary');

INSERT INTO work (title, release_date, work_type, summary, runtime, language, network, status) VALUES
    ('The Simpsons', '1989-12-17', 2, 'The satiric adventures of a working-class family in the misfit city of Springfield.', 30, 'English', 'FOX', 'Running'),
    ('The Simpsons Movie', '2007-07-27', 1, 'After Homer accidentally pollutes the town''s water supply, Springfield is encased in a gigantic dome by the EPA and the Simpsons are declared fugitives.', 87, 'English', 'FOX', 'Released'),
    ('The Simpsons: Treehouse of Horror', '1990-10-25', 2, 'The Simpsons host a special Halloween show, featuring three separate segments.', 30, 'English', 'FOX', 'Running'),
    ('The Simpsons: Behind the Laughter', '2000-05-21', 2, 'The Simpsons host a special behind the scenes show, featuring the actors who play the characters.', 30, 'English', 'FOX', 'Running'),
    ('The Simpsons: 138th Episode Spectacular', '1995-12-03', 2, 'Troy McClure hosts a special episode of The Simpsons, looking back at the first 138 episodes.', 30, 'English', 'FOX', 'Running'),
    ('The Simpsons: The Longest Daycare', '2012-07-13', 2, 'Maggie Simpson spends the day in the Ayn Rand School for Tots.', 5, 'English', 'FOX', 'Running'),
    ('The Simpsons: The Good, the Bart, and the Loki', '2021-07-07', 2, 'Loki joins forces with Bart Simpson to fight against his toughest opponent yet.', 5, 'English', 'Disney+', 'Running');


INSERT INTO rating (work_id, rating_value, rating_source, rating_date) VALUES
    (1, 7, 'stetson', '2023-07-08'),
    (2, 6, 'stetson', '2023-07-09'),
    (3, 4, 'stetson', '2023-07-10');

