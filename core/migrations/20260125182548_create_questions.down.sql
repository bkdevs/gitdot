-- Drop tables in reverse order of creation (due to foreign key dependencies)
DROP TABLE IF EXISTS comments;
DROP TABLE IF EXISTS answers;
DROP TABLE IF EXISTS questions;
