CREATE TABLE IF NOT EXISTS reminders (
  reminder_id SERIAL PRIMARY KEY,
  due_at TIMESTAMPTZ NOT NULL,
  created_at TIMESTAMPTZ NOT NULL,
  channel_snowflake BIGINT NOT NULL,
  message_snowflake BIGINT NOT NULL,
  reminder_message TEXT
);