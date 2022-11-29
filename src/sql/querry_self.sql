SELECT id,
    name,
    avatar_url,
    new,
    sessions.created
FROM users
    JOIN sessions ON sessions.user_id = users.id
WHERE sessions.session_id = ?;