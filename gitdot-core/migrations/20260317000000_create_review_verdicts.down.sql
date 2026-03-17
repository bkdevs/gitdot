DROP TABLE IF EXISTS review_verdicts;
DROP TYPE IF EXISTS verdict;

-- Revert enum values to lowercase
ALTER TYPE diff_status RENAME VALUE 'changes_requested' TO 'changesrequested';
ALTER TYPE review_status RENAME VALUE 'in_progress' TO 'inprogress';
