CREATE TABLE owners (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(150) NOT NULL,
    phone VARCHAR(50),
    email VARCHAR(255),
    address TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_owners_name ON owners(name);

ALTER TABLE farms ADD COLUMN owner_id BIGINT REFERENCES owners(id);
