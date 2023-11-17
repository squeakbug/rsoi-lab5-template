// @generated automatically by Diesel CLI.

diesel::table! {
    airport (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Nullable<Varchar>,
        #[max_length = 255]
        city -> Nullable<Varchar>,
        #[max_length = 255]
        country -> Nullable<Varchar>,
    }
}

diesel::table! {
    flight (id) {
        id -> Int4,
        #[max_length = 20]
        flight_number -> Varchar,
        datetime -> Timestamptz,
        from_airport_id -> Nullable<Int4>,
        to_airport_id -> Nullable<Int4>,
        price -> Int4,
    }
}

diesel::table! {
    privilege (id) {
        id -> Int4,
        #[max_length = 80]
        username -> Varchar,
        #[max_length = 80]
        status -> Varchar,
        balance -> Nullable<Int4>,
    }
}

diesel::table! {
    privilege_history (id) {
        id -> Int4,
        privilege_id -> Nullable<Int4>,
        ticket_uid -> Uuid,
        datetime -> Timestamp,
        balance_diff -> Int4,
        #[max_length = 20]
        operation_type -> Varchar,
    }
}

diesel::table! {
    ticket (id) {
        id -> Int4,
        ticket_uid -> Uuid,
        #[max_length = 80]
        username -> Varchar,
        #[max_length = 20]
        flight_number -> Varchar,
        price -> Int4,
        #[max_length = 20]
        status -> Varchar,
    }
}

diesel::joinable!(privilege_history -> privilege (privilege_id));

diesel::allow_tables_to_appear_in_same_query!(airport, flight, privilege, privilege_history, ticket,);
