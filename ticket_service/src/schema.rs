// @generated automatically by Diesel CLI.

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
