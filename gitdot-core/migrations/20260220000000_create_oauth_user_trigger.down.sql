DROP TRIGGER IF EXISTS on_auth_oauth_user_created ON auth.users;
DROP FUNCTION IF EXISTS public.handle_oauth_user_created();
