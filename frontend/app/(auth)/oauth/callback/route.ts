import { redirect } from "next/navigation";
import type { NextRequest } from "next/server";
import { createSupabaseClient } from "@/lib/supabase";

export async function GET(request: NextRequest) {
  const { searchParams } = new URL(request.url);
  const code = searchParams.get("code");

  if (!code) {
    redirect("/login");
    return;
  }

  const supabase = await createSupabaseClient();
  const { error } = await supabase.auth.exchangeCodeForSession(code);

  if (error) {
    redirect("/login");
    return;
  }

  // TODO: for new account, redirect to /onboarding
  redirect("/home");
}
