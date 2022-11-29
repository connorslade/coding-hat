UPDATE users
SET new = 0
WHERE id = (
        SELECT user_id
        FROM sessions
        WHERE session_id = ?1
    );