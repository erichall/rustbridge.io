-- Your SQL goes here
CREATE TABLE workshops (
  id SERIAL PRIMARY KEY NOT NULL,
  name VARCHAR NOT NULL,
  organizer INTEGER NOT NULL REFERENCES users(id),
  description VARCHAR NOT NULL,
  location VARCHAR NOT NULL,
  event_date TIMESTAMP NOT NULL,
  start_time TIMESTAMP NOT NULL,
  end_time TIMESTAMP NOT NULL,
  private BOOLEAN NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

SELECT diesel_manage_updated_at('workshops');
