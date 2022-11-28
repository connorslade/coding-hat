CREATE TABLE IF NOT EXISTS sessions (
   created INTEGER NOT NULL, -- Create epoch
   user_id TEXT NOT NULL,    -- user.id
   session_id TEXT NOT NULL  -- Random session ID
)