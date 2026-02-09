DROP TRIGGER IF EXISTS on_auth_email_confirmed ON auth.users;
DROP FUNCTION IF EXISTS public.handle_email_confirmed();
