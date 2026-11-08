CREATE TABLE farm_photos (
    id BIGSERIAL PRIMARY KEY,
    farm_id BIGINT NOT NULL REFERENCES farms(id) ON DELETE CASCADE,
    filename VARCHAR(255) NOT NULL,
    filepath VARCHAR(500) NOT NULL,
    mime_type VARCHAR(100),
    size_bytes BIGINT,
    uploaded_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    lon NUMERIC(10, 6),
    lat NUMERIC(10, 6)
);

CREATE INDEX idx_farm_photos_farm_id ON farm_photos(farm_id);
CREATE INDEX idx_farm_photos_uploaded_at ON farm_photos(uploaded_at);
