CREATE TYPE farm_status AS ENUM ('pending', 'approved', 'rejected');

ALTER TABLE farms ADD COLUMN status farm_status NOT NULL DEFAULT 'pending';
ALTER TABLE farms ADD COLUMN approved_by BIGINT REFERENCES users(id);
ALTER TABLE farms ADD COLUMN approved_at TIMESTAMPTZ;
