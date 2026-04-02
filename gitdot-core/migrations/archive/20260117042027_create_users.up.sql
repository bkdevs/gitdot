-- Create users table
CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    email VARCHAR(255) NOT NULL UNIQUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create index for better query performance
CREATE INDEX idx_users_name ON users(name);

-- Supabase trigger: auto-create user when auth.users row is created
-- Disabled for local dev — requires Supabase auth schema
-- CREATE OR REPLACE FUNCTION public.handle_new_auth_user()
-- RETURNS TRIGGER AS $$
-- BEGIN
--     INSERT INTO public.users (id, name, email, created_at)
--     VALUES (
--         NEW.id,
--         COALESCE(NEW.raw_user_meta_data->>'name', SPLIT_PART(NEW.email, '@', 1)),
--         NEW.email,
--         NOW()
--     )
--     ON CONFLICT (id) DO NOTHING;
--     RETURN NEW;
-- END;
-- $$ LANGUAGE plpgsql SECURITY DEFINER;
--
-- CREATE TRIGGER on_auth_user_created
--     AFTER INSERT ON auth.users
--     FOR EACH ROW
--     EXECUTE FUNCTION public.handle_new_auth_user();
