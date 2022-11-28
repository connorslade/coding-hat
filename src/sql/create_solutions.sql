CREATE TABLE IF NOT EXISTS solutions (
    user_id TEXT NOT NULL,       -- user.id
    problem_id INTEGER NOT NULL, -- Problem id (EX: 200079)
    state INTEGER NOT NULL,      -- (0: Not Started, 1: In Progress, 2: Complete)
    code TEXT NOT NULL,          -- Accutal Code
    language TEXT NOT NULL,      -- Lang used (python, java)
    created INTEGER NOT NULL,    -- Date created
    UNIQUE(user_id, problem_id, language)
)