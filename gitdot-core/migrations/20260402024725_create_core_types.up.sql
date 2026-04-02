CREATE TYPE core.auth_provider AS ENUM ('email', 'github');
CREATE TYPE core.organization_role AS ENUM ('admin', 'member');
CREATE TYPE core.repository_owner_type AS ENUM ('user', 'organization');
CREATE TYPE core.repository_visibility AS ENUM ('public', 'private');
CREATE TYPE core.review_status AS ENUM ('draft', 'in_progress', 'closed');
CREATE TYPE core.diff_status AS ENUM ('open', 'changes_requested', 'approved', 'merged');
CREATE TYPE core.verdict AS ENUM ('approved', 'changes_requested');
CREATE TYPE core.comment_side AS ENUM ('old', 'new');
CREATE TYPE core.webhook_event_type AS ENUM ('push');
