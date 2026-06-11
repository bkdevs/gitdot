CREATE TABLE core.user_followers (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    follower_id UUID NOT NULL REFERENCES core.users(id) ON DELETE CASCADE,
    following_id UUID NOT NULL REFERENCES core.users(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CHECK (follower_id <> following_id),
    UNIQUE(follower_id, following_id)
);

CREATE INDEX idx_user_followers_follower_id ON core.user_followers (follower_id);
CREATE INDEX idx_user_followers_following_id ON core.user_followers (following_id);
