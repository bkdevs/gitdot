CREATE TYPE ci.runner_owner_type AS ENUM ('user', 'organization');
CREATE TYPE ci.task_status AS ENUM ('pending', 'assigned', 'running', 'success', 'failure');
CREATE TYPE ci.build_status AS ENUM ('running', 'success', 'failure');
CREATE TYPE ci.build_trigger AS ENUM ('pull_request', 'push_to_main');
