-- Your SQL goes here
create table fund_history(
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    nav FLOAT8,
    date DATE,
    scheme_code VARCHAR,
    fund_id uuid,
    constraint fk_fund_history_fund FOREIGN KEY(fund_id) REFERENCES fund(id)
)
