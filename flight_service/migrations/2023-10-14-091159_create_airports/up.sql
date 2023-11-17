CREATE TABLE airport
(
    id      SERIAL PRIMARY KEY,
    name    VARCHAR(255),
    city    VARCHAR(255),
    country VARCHAR(255)
);

INSERT INTO airport (name, city, country) values ('Шереметьево', 'Москва', 'Россия');
INSERT INTO airport (name, city, country) values ('Пулково', 'Санкт-Петербург', 'Россия');
