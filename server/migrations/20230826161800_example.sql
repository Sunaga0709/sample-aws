-- Add migration script here
CREATE TABLE IF NOT EXISTS example (
	example_id CHAR(36) PRIMARY KEY,
	created_at BIGINT NOT NULL
);