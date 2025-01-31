diesel::table! {
    consumer_facts (id) {
        id -> Int4,
        first_name -> Varchar,
        last_name -> Varchar,
        email -> Varchar,
        date_of_birth -> Varchar,
        address -> Varchar,
        phone_number -> Varchar,
        consumer_state -> Varchar,
        institution_names -> Array<Text>,
    }
}

diesel::table! {
    credit_facts (id) {
        id -> Int4,
        consumer_id -> Int4,
        amount -> Float8,
        credit_type -> Varchar,
        application_datetime -> Varchar,
        credit_state -> Varchar,
    }
}
