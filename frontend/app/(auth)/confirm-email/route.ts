import type { EmailOtpType } from "@supabase/supabase-js";
import { redirect } from "next/navigation";
import type { NextRequest } from "next/server";
import { createSupabaseClient } from "@/lib/supabase";

/**
 * a public GET endpoint that is linked to by the supabase email confirmation linked
 * this executes the pkce flow (supabase sets a verifier in cookies in signup), saves a session, and redirects to the user
 */
export async function GET(request: NextRequest) {
  const { searchParams } = new URL(request.url);
  const token_hash = searchParams.get("token_hash");
  const type = searchParams.get("type") as EmailOtpType | null;
  const next = searchParams.get("next") ?? "/";

  if (token_hash && type) {
    const supabase = await createSupabaseClient();

    // this call invokes _saveSession and sets cookies
    const { error } = await supabase.auth.verifyOtp({ type, token_hash });

    // redirects preserve cookies
    if (!error) {
      redirect(next);
    }
  }

  redirect("/error");
}
