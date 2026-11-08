ALTER TABLE farms ADD COLUMN boundary GEOMETRY(Polygon, 4326);
ALTER TABLE farms ADD COLUMN area_hectares NUMERIC(10, 4);
