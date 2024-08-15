-- The database initialization script is used for defining your local schema as well as postgres
-- running within a docker container, where we'll copy this file over and run on startup

DO
$$
    BEGIN
        IF
            NOT EXISTS (SELECT 1 FROM pg_database WHERE datname = 'hop_query') THEN
            CREATE DATABASE hop_query;
        END IF;
    END
$$;

\c hop_query;

DROP TABLE IF EXISTS breweries CASCADE;

CREATE TABLE breweries (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    location VARCHAR(100) NOT NULL,
    year_established INTEGER,
    description TEXT,
    website VARCHAR(255),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

DROP TABLE IF EXISTS beer_styles CASCADE;

CREATE TABLE beer_styles (
    id SERIAL PRIMARY KEY,
    name VARCHAR(50) NOT NULL,
    description TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

DROP TABLE IF EXISTS beers CASCADE;

CREATE TABLE beers (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    brewery_id INTEGER NOT NULL REFERENCES breweries (id),
    style_id INTEGER NOT NULL REFERENCES beer_styles (id),
    abv NUMERIC(3, 1) NOT NULL,
    ibu INTEGER,
    description TEXT,
    is_seasonal BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

DROP TABLE IF EXISTS ingredients CASCADE;

CREATE TABLE ingredients (
    id SERIAL PRIMARY KEY,
    name VARCHAR(50) NOT NULL,
    type VARCHAR(20) NOT NULL, -- e.g., 'hop', 'malt', 'yeast', 'adjunct'
    description TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

DROP TABLE IF EXISTS beer_ingredients CASCADE;

CREATE TABLE beer_ingredients (
    beer_id INTEGER NOT NULL REFERENCES beers (id),
    ingredient_id INTEGER NOT NULL REFERENCES ingredients (id),
    amount VARCHAR(50), -- e.g., '2 oz', '5 lbs'
    PRIMARY KEY (beer_id, ingredient_id)
);

DROP TABLE IF EXISTS reviews CASCADE;

CREATE TABLE reviews (
    id SERIAL PRIMARY KEY,
    beer_id INTEGER NOT NULL REFERENCES beers (id),
    user_name VARCHAR(50) NOT NULL,
    rating INTEGER NOT NULL CHECK (
        rating >= 1
        AND rating <= 5
    ),
    comment TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Create indexes for better query performance
CREATE INDEX idx_beers_breweries ON beers (brewery_id);

CREATE INDEX idx_beer_styles ON beers (style_id);

CREATE INDEX idx_beer_ingredients_beers ON beer_ingredients (beer_id);

CREATE INDEX idx_beer_ingredients_ingredients ON beer_ingredients (ingredient_id);

CREATE INDEX idx_reviews_beers ON reviews (beer_id);

-- Create a function to update the 'updated_at' timestamp
CREATE OR REPLACE FUNCTION update_modified_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = now();
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Create triggers to automatically update the 'updated_at' timestamp
CREATE TRIGGER update_brewery_modtime BEFORE
UPDATE ON breweries FOR EACH ROW
EXECUTE FUNCTION update_modified_column ();

CREATE TRIGGER update_beer_modtime BEFORE
UPDATE ON beers FOR EACH ROW
EXECUTE FUNCTION update_modified_column ();