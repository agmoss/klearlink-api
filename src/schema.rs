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
        institution_name -> Nullable<Text>,
        amount -> Float8,
        #[max_length = 10]
        credit_type -> Varchar,
        application_datetime -> Timestamp,
        originated_datetime -> Nullable<Timestamp>,
        payment_due_date -> Nullable<Timestamp>,
        payment_due_amount -> Nullable<Float8>,
        #[max_length = 20]
        credit_state -> Varchar,
        #[max_length = 2]
        consumer_information_indicator -> Nullable<Varchar>,
        user_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
        total_installments -> Nullable<Int4>,
        paid_installments -> Nullable<Int4>,
        installment_amount -> Nullable<Float8>,
        #[max_length = 45]
        ip_address -> Nullable<Varchar>,
    }
}

diesel::table! {
    consumer_credit_events (id) {
        id -> Int4,
        #[max_length = 100]
        consumer_credit_id -> Varchar,
        #[max_length = 50]
        event_type -> Varchar,
        event_data -> Jsonb,
        created_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 100]
        username -> Varchar,
        api_key -> Uuid,
        #[max_length = 100]
        role -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(consumer_credit -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(consumer_credit, consumer_credit_events, users,);
