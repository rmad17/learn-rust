// @generated automatically by Diesel CLI.

diesel::table! {
    fund (id) {
        id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        scheme_code -> Varchar,
        scheme_name -> Varchar,
        scheme_category -> Nullable<Varchar>,
        fund_house -> Varchar,
        status -> Nullable<Varchar>,
        description -> Nullable<Text>,
        current_nav -> Nullable<Float8>,
        started_at -> Nullable<Timestamptz>,
        last_refreshed_at -> Nullable<Timestamptz>,
        started_on -> Nullable<Date>,
        ended_on -> Nullable<Date>,
    }
}

diesel::table! {
    fund_history (id) {
        id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        nav -> Nullable<Float8>,
        date -> Nullable<Date>,
        scheme_code -> Nullable<Varchar>,
        fund_id -> Nullable<Uuid>,
    }
}

diesel::joinable!(fund_history -> fund (fund_id));

diesel::allow_tables_to_appear_in_same_query!(fund, fund_history,);
