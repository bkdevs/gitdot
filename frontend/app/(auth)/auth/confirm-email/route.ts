import type { EmailOtpType } from "@supabase/supabase-js";
import { redirect } from "next/navigation";
import type { NextRequest } from "next/server";
import { createUser } from "@/lib/dal";
import { createSupabaseClient } from "@/lib/supabase";

/**
 * a public GET endpoint that is linked to by the supabase email confirmation linked
 * this executes the pkce flow (supabase sets a verifier in cookies in signup), saves a session, and redirects to the user
 */
export async function GET(request: NextRequest) {
  const { searchParams } = new URL(request.url);
  const token_hash = searchParams.get("token_hash");

  if (token_hash) {
    const supabase = await createSupabaseClient();
    const { data, error } = await supabase.auth.verifyOtp({ type: "email", token_hash });

    // redirects preserve cookies
    if (!error) {
      const username = data.user?.user_metadata?.username;

      // TODO: technically possible user name was claimed by another user between email confirmation delays
      // should show some error and let user select another username
      if (username) {
        await createUser(username);
      }
      redirect("/onboarding");
    }
  }

  redirect("/error");
}
