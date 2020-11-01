-- Add migration script here
CREATE TABLE IF NOT EXISTS message (
    key varchar(61) NOT NULL,
    type varchar(16) NOT NULL,
    data varchar(6000) NOT NULL,
    sender varchar(30) NOT NULL,
    time timestamp default current_timestamp NOT NULL,
    PRIMARY KEY(key, time)
)