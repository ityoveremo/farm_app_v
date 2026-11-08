CREATE TABLE farm_crops (
    farm_id BIGINT NOT NULL REFERENCES farms(id) ON DELETE CASCADE,
    crop_id BIGINT NOT NULL REFERENCES crops(id) ON DELETE CASCADE,
    PRIMARY KEY (farm_id, crop_id)
);
