// @generated automatically by Diesel CLI.

diesel::table! {
    consumer_credit (id) {
        id -> Int4,
        consumer_credit_id -> Varchar,
        first_name -> Varchar,
        last_name -> Varchar,
        email -> Varchar,
        date_of_birth -> Varchar,
        address -> Varchar,
        phone_number -> Varchar,
        consumer_state -> Varchar,
        institution_names -> Array<Nullable<Text>>,
        amount -> Float8,
        credit_type -> Varchar,
        application_datetime -> Varchar,
        credit_state -> Varchar,
        tenant -> Varchar,
    }
}
