-- Create table to manage migrations
CREATE TABLE IF NOT EXISTS migrations (
  id TEXT PRIMARY KEY,
  description TEXT,
  previous_id TEXT,
  applied_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY (previous_id) REFERENCES migrations(id)
);


-- Store database version by migration id
CREATE TABLE IF NOT EXISTS current_migrations (
  id TEXT PRIMARY KEY,
  FOREIGN KEY (id) REFERENCES migrations(id)
);


-- Insert the initial migration
INSERT INTO migrations (id, description, previous_id) VALUES
('v0001_437d049d-4932-4326-906f-90b761aac972', 'first run', NULL);


-- Set the current migration to this one
DELETE FROM current_migrations ;
INSERT INTO current_migrations (id) VALUES
('v0001_437d049d-4932-4326-906f-90b761aac972');


-- Create a table to store sessions
CREATE TABLE IF NOT EXISTS sessions (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  location TEXT NOT NULL,
  datetime DATETIME NOT NULL,
  duration INTEGER NOT NULL,
  rating INTEGER NOT NULL,
  wave_height REAL NOT NULL
);
