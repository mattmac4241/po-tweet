CREATE TABLE auth_tokens (
  id SERIAL PRIMARY KEY,
  key VARCHAR NOT NULL UNIQUE,
  expires_at TIMESTAMP NOT NULL,
  user_id INT references USERS(ID)
)
