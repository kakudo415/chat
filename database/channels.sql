CREATE TABLE IF NOT EXISTS channels (
  "id"         UUID PRIMARY KEY,
  "name"       VARCHAR NOT NULL,
  "created_at" TIMESTAMP NOT NULL DEFAULT now()
);
