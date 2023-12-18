CREATE TABLE IF NOT EXISTS messages (
  "id"         UUID PRIMARY KEY,
  "text"       TEXT NOT NULL,
  "channel_id" UUID NOT NULL,
  "created_at" TIMESTAMP NOT NULL DEFAULT now()
);
