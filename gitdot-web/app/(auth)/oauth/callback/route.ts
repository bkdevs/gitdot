import { redirect } from "next/navigation";
import type { NextRequest } from "next/server";
import { exchangeGitHubCode } from "@/lib/auth";

export async function GET(request: NextRequest) {
  const { searchParams } = new URL(request.url);
  const code = searchParams.get("code");
  const state = searchParams.get("state");

  if (!code || !state) {
    redirect("/login");
    return;
  }

  const result = await exchangeGitHubCode(code, state);
  if (!result) {
    redirect("/login");
    return;
  }

  redirect(result.is_new ? "/onboarding" : "/home");
}
