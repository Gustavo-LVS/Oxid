CREATE TABLE messages (
    id         UUID PRIMARY KEY,
    channel_id UUID NOT NULL REFERENCES channels(id) ON DELETE CASCADE,
    author_id  UUID NOT NULL REFERENCES users(id),
    content    TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
-- UUID v7 é ordenado por tempo: paginação por id.
CREATE INDEX messages_channel_idx ON messages (channel_id, id DESC);
