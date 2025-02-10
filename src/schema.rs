// @generated automatically by Diesel CLI.

diesel::table! {
    consumer_credit (id) {
        id -> Int4,
        #[max_length = 100]
        consumer_credit_id -> Varchar,
        #[max_length = 100]
        first_name -> Varchar,
        #[max_length = 100]
        last_name -> Varchar,
        #[max_length = 320]
        email -> Varchar,
        date_of_birth -> Date,
        address -> Text,
        #[max_length = 20]
        phone_number -> Varchar,
        #[max_length = 11]
        sin_ssn -> Nullable<Varchar>,
        institution_names -> Array<Nullable<Text>>,
        amount -> Numeric,
        #[max_length = 10]
        credit_type -> Varchar,
        application_datetime -> Timestamp,
        #[max_length = 20]
        credit_state -> Varchar,
        #[max_length = 50]
        tenant -> Varchar,
        user_id -> Int4,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 100]
        username -> Varchar,
        #[max_length = 100]
        api_key -> Varchar,
    }
}

diesel::joinable!(consumer_credit -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(consumer_credit, users,);
