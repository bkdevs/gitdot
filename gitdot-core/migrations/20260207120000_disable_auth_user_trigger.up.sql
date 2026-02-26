-- Disable the trigger that auto-creates a user when a Supabase auth user is created
DROP TRIGGER IF EXISTS on_auth_user_created ON auth.users;

-- Drop the associated function
DROP FUNCTION IF EXISTS public.handle_new_auth_user();
