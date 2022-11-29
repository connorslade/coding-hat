INSERT INTO users
VALUES (?1, ?2, ?3, ?4, strftime('%s', 'now'), 1) ON CONFLICT DO
UPDATE
SET name = ?2,
    avatar_url = ?3,
    access_token = ?4;