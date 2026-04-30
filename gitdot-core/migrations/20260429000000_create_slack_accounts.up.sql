CREATE TABLE auth.slack_accounts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    gitdot_user_id UUID NOT NULL REFERENCES core.users(id) ON DELETE CASCADE,
    slack_user_id VARCHAR(64) NOT NULL,
    slack_team_id VARCHAR(64) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (slack_team_id, slack_user_id)
);

CREATE INDEX idx_slack_accounts_gitdot_user_id ON auth.slack_accounts (gitdot_user_id);
