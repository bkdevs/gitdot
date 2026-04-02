drop function if exists public.custom_access_token_hook;
revoke execute on function public.custom_access_token_hook from supabase_auth_admin;
revoke select on table public.users from supabase_auth_admin;
revoke select on table public.organizations from supabase_auth_admin;
revoke select on table public.organization_members from supabase_auth_admin;
