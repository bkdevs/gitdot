ALTER TYPE core.webhook_event_type ADD VALUE IF NOT EXISTS 'review_publish';
ALTER TYPE core.webhook_event_type ADD VALUE IF NOT EXISTS 'review_update';

CREATE TABLE core.slack_webhooks (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES core.users(id) ON DELETE CASCADE,
    repository_id UUID NOT NULL REFERENCES core.repositories(id) ON DELETE CASCADE,
    events core.webhook_event_type[] NOT NULL,
    slack_user_id VARCHAR(64) NOT NULL,
    slack_team_id VARCHAR(64) NOT NULL,
    slack_channel_id VARCHAR(64) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (repository_id, slack_team_id, slack_channel_id)
);

CREATE INDEX idx_slack_webhooks_user_id ON core.slack_webhooks (user_id);
CREATE INDEX idx_slack_webhooks_repository_id ON core.slack_webhooks (repository_id);
