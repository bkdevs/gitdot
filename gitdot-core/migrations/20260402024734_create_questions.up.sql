CREATE TABLE core.questions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    number INTEGER NOT NULL,
    author_id UUID NOT NULL REFERENCES core.users(id) ON DELETE CASCADE,
    repository_id UUID NOT NULL REFERENCES core.repositories(id) ON DELETE CASCADE,
    title VARCHAR(255) NOT NULL,
    body TEXT NOT NULL,
    upvote INTEGER NOT NULL DEFAULT 0,
    impression INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(repository_id, number)
);

CREATE INDEX idx_questions_author_id ON core.questions (author_id);
CREATE INDEX idx_questions_repository_id ON core.questions (repository_id);
CREATE INDEX idx_questions_created_at ON core.questions (created_at);

CREATE TABLE core.answers (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    question_id UUID NOT NULL REFERENCES core.questions(id) ON DELETE CASCADE,
    author_id UUID NOT NULL REFERENCES core.users(id) ON DELETE CASCADE,
    body TEXT NOT NULL,
    upvote INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_answers_question_id ON core.answers (question_id);
CREATE INDEX idx_answers_author_id ON core.answers (author_id);
CREATE INDEX idx_answers_created_at ON core.answers (created_at);

CREATE TABLE core.comments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    parent_id UUID NOT NULL,
    author_id UUID NOT NULL REFERENCES core.users(id) ON DELETE CASCADE,
    body TEXT NOT NULL,
    upvote INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_comments_parent_id ON core.comments (parent_id);
CREATE INDEX idx_comments_author_id ON core.comments (author_id);
CREATE INDEX idx_comments_created_at ON core.comments (created_at);

CREATE TABLE core.votes (
    user_id UUID NOT NULL REFERENCES core.users(id) ON DELETE CASCADE,
    target_id UUID NOT NULL,
    value SMALLINT NOT NULL CHECK (value IN (-1, 1)),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY(user_id, target_id)
);

CREATE INDEX idx_votes_target_id ON core.votes (target_id);
