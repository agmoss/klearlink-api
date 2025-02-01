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
        institution_names -> Array<Nullable<Varchar>>,
        amount -> Numeric,
        #[max_length = 10]
        credit_type -> Varchar,
        application_datetime -> Timestamp,
        #[max_length = 20]
        credit_state -> Varchar,
        #[max_length = 50]
        tenant -> Varchar,
    }
}
