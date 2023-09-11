-- Add migration script here
CREATE TABLE IF NOT EXISTS user (
	user_id CHAR(36) PRIMARY KEY,
	auth_provider_id VARCHAR(255) NOT NULL,
	name VARCHAR(255) NOT NULL,
	birthday BIGINT NOT NULL,
	email VARCHAR(255) NOT NULL,
	created_at BIGINT NOT NULL,
	updated_at BIGINT NOT NULL,
	is_deleted BOOLEAN NOT NULL DEFAULT FALSE
);
