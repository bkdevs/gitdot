import "server-only";

import { createServerClient } from "@supabase/ssr";
import type { JwtPayload } from "@supabase/supabase-js";
import { PHASE_PRODUCTION_BUILD } from "next/constants";
import { cookies } from "next/headers";
import { type NextRequest, NextResponse } from "next/server";

/**
 * we do *not* expose supabase environment variables in the client
 * we use supabase in server functions *only* and in the middleware for auth.
 */
function getSupabaseConfig() {
  const supabaseUrl = process.env.SUPABASE_URL;
  const supabaseKey = process.env.SUPABASE_PUBLISHABLE_KEY;

  if (!supabaseUrl || !supabaseKey) {
    throw new Error("SUPABASE_URL and SUPABASE_PUBLISHABLE_KEY must be set");
  }

  return { supabaseUrl, supabaseKey };
}

/**
 * this function should be called once per request
 * as it is lightweight (just configures a fetch) and necessary (runs on serverless edge functions)
 */
export async function createSupabaseClient() {
  const { supabaseUrl, supabaseKey } = getSupabaseConfig();
  const cookieStore = await cookies();

  return createServerClient(supabaseUrl, supabaseKey, {
    cookies: {
      getAll() {
        return cookieStore.getAll();
      },
      setAll(cookiesToSet) {
        try {
          cookiesToSet.forEach(({ name, value, options }) => {
            cookieStore.set(name, value, { ...options, httpOnly: true });
          });
        } catch (error) {
          console.error(error);
        }
      },
    },
  });
}

/**
 * invoked in middleware, checks for session and also updates session if expired
 */
export async function updateSession(request: NextRequest) {
  const { supabaseUrl, supabaseKey } = getSupabaseConfig();
  let supabaseResponse = NextResponse.next({ request });

  const supabase = createServerClient(supabaseUrl, supabaseKey, {
    cookies: {
      getAll() {
        return request.cookies.getAll();
      },
      setAll(cookiesToSet) {
        cookiesToSet.forEach(({ name, value }) => {
          request.cookies.set(name, value);
        });
        supabaseResponse = NextResponse.next({
          request,
        });
        cookiesToSet.forEach(({ name, value, options }) => {
          supabaseResponse.cookies.set(name, value, options);
        });
      },
    },
  });

  // checks session & refreshes if necessary, setting both cookies & supabaseResponse
  const { data } = await supabase.auth.getClaims();
  const user = data?.claims;

  const isAuthPath = ["/login", "/signup"].includes(request.nextUrl.pathname);
  if (user && isAuthPath) {
    return NextResponse.redirect(new URL("/home", request.nextUrl));
  }

  // return the supabaseResponse object as-is, this is required to ensure that cookies are in sync between the server and client.
  return supabaseResponse;
}

/**
 * for use in server-components that need user identity info
 */
export async function getClaims(): Promise<JwtPayload | null> {
  const supabase = await createSupabaseClient();
  const { data } = await supabase.auth.getClaims();

  return data?.claims || null;
}

/**
 * for use when the full session (including access token) is needed
 */
export async function getSession() {
  if (process.env.NEXT_PHASE === PHASE_PRODUCTION_BUILD) {
    // constructing the supabase client awaits cookies, which causes failures if done at build time
    // any page generated in buidl time is unauthenticated (e.g., public), so return null
    return null;
  }

  const supabase = await createSupabaseClient();
  const { data } = await supabase.auth.getSession();

  return data.session;
}
