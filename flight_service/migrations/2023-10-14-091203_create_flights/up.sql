CREATE TABLE flight
(
    id              SERIAL PRIMARY KEY,
    flight_number   VARCHAR(20)              NOT NULL,
    datetime        TIMESTAMP WITH TIME ZONE NOT NULL,
    from_airport_id INT REFERENCES airport (id),
    to_airport_id   INT REFERENCES airport (id),
    price           INT                      NOT NULL
);

INSERT INTO flight (flight_number, datetime, from_airport_id, to_airport_id, price)
    values ('AFL031', cast('2021-10-08 20:00:00' as timestamp with time zone), 2, 1, 1500);
