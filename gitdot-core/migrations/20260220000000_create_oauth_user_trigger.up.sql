-- Create trigger function to auto-create user when an OAuth user is created
-- OAuth providers (e.g., GitHub) create auth.users rows with email_confirmed_at already set,
-- so the existing on_auth_email_confirmed UPDATE trigger won't fire for them.
CREATE OR REPLACE FUNCTION public.handle_oauth_user_created()
RETURNS TRIGGER AS $$
BEGIN
    -- Only fire for users who arrive with email already confirmed (OAuth providers)
    IF NEW.email_confirmed_at IS NOT NULL THEN
        INSERT INTO public.users (id, name, email, created_at)
        VALUES (
            NEW.id,
            'user_' || LPAD(FLOOR(RANDOM() * 1000000)::TEXT, 6, '0'),
            NEW.email,
            NOW()
        )
        ON CONFLICT (id) DO NOTHING;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql SECURITY DEFINER;

CREATE TRIGGER on_auth_oauth_user_created
    AFTER INSERT ON auth.users
    FOR EACH ROW
    EXECUTE FUNCTION public.handle_oauth_user_created();
