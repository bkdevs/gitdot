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

  if (!token_hash) {
    return;
  }

  const supabase = await createSupabaseClient();
  const { error } = await supabase.auth.verifyOtp({
    type: "email",
    token_hash,
  });

  if (error) {
    // TODO: resend email link
    redirect("/error");
    return;
  }

  // redirects preserve cookies, so the user is logged in
  redirect("/onboarding");
}
