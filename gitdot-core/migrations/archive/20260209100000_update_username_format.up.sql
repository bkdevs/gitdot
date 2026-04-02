-- Update trigger function to remove underscore between 'user' and generated numbers
CREATE OR REPLACE FUNCTION public.handle_email_confirmed()
RETURNS TRIGGER AS $$
BEGIN
    -- Only fire when email_confirmed_at transitions from NULL to non-NULL
    IF OLD.email_confirmed_at IS NULL AND NEW.email_confirmed_at IS NOT NULL THEN
        INSERT INTO public.users (id, name, email, created_at)
        VALUES (
            NEW.id,
            'user' || LPAD(FLOOR(RANDOM() * 1000000)::TEXT, 6, '0'),
            NEW.email,
            NOW()
        )
        ON CONFLICT (id) DO NOTHING;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql SECURITY DEFINER;
