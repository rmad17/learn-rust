CREATE TABLE fund (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    scheme_code VARCHAR NOT NULL,
    scheme_name VARCHAR NOT NULL,
    scheme_category VARCHAR,
    fund_house VARCHAR NOT NULL,
    status VARCHAR,
    description TEXT,
    current_nav FLOAT8,
    started_at TIMESTAMPTZ,
    last_refreshed_at TIMESTAMPTZ,
    started_on DATE,
    ended_on DATE
)
