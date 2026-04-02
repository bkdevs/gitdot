-- Drop trigger
DROP TRIGGER IF EXISTS on_auth_user_created ON auth.users;

-- Drop trigger function
DROP FUNCTION IF EXISTS public.handle_new_auth_user();

-- Drop indexes
DROP INDEX IF EXISTS idx_users_email;

-- Drop users table
DROP TABLE IF EXISTS users;
