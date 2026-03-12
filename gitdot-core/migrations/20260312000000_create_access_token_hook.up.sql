create or replace function public.custom_access_token_hook(event jsonb)
returns jsonb
language plpgsql
security definer
as $$
  declare
    claims jsonb;
    user_name text;
    user_orgs jsonb;
  begin
    -- Look up username (public.users.id == auth.users.id)
    select name into user_name
    from public.users
    where id = (event->>'user_id')::uuid;

    -- Build orgs array: ["orgname:role", ...]
    select coalesce(
      jsonb_agg(o.name || ':' || om.role::text),
      '[]'::jsonb
    )
    into user_orgs
    from public.organization_members om
    join public.organizations o on o.id = om.organization_id
    where om.user_id = (event->>'user_id')::uuid;

    claims := event->'claims';

    -- Ensure user_metadata object exists
    if jsonb_typeof(claims->'user_metadata') is null then
      claims := jsonb_set(claims, '{user_metadata}', '{}');
    end if;

    -- Inject username and orgs
    claims := jsonb_set(claims, '{user_metadata,username}', to_jsonb(coalesce(user_name, '')));
    claims := jsonb_set(claims, '{user_metadata,orgs}', user_orgs);

    event := jsonb_set(event, '{claims}', claims);
    return event;
  end;
$$;

-- Allow supabase_auth_admin to call the hook
grant execute
  on function public.custom_access_token_hook
  to supabase_auth_admin;

-- Allow supabase_auth_admin to read needed tables
grant select on table public.users to supabase_auth_admin;
grant select on table public.organizations to supabase_auth_admin;
grant select on table public.organization_members to supabase_auth_admin;

-- Revoke direct table access from anon/public (RLS handles authenticated)
revoke select on table public.users from anon, public;
revoke select on table public.organizations from anon, public;
revoke select on table public.organization_members from anon, public;
