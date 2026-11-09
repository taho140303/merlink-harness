-- Add durable soft-delete state without changing or removing existing sessions.
ALTER TABLE sessions ADD COLUMN deleted_at TEXT;
CREATE INDEX idx_sessions_deleted_at ON sessions(deleted_at);
PRAGMA user_version = 8;
