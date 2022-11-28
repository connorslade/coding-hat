CREATE TABLE IF NOT EXISTS attempts (
    user_id TEXT NOT NULL,          -- user.id
    problem_id INTEGER NOT NULL,    -- Problem id (EX: 200079)
    language TEXT NOT NULL,         -- Lang used (python, java)
    date INTEGER NOT NULL,          -- Event epoch
    tests_passing INTEGER NOT NULL, -- Count of tests that passing
)

-- Refrence problem file to get total test count