import { redirect } from "next/navigation";
import type { NextRequest } from "next/server";
import { verifyAuthCode } from "@/lib/auth";

export async function GET(request: NextRequest) {
  const { searchParams } = new URL(request.url);
  const code = searchParams.get("code");
  if (!code) redirect("/login");

  const result = await verifyAuthCode(code);
  if (!result) redirect("/login");

  const redirectTo = searchParams.get("redirect") || "/home";
  redirect(redirectTo);
}
