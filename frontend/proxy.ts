import type { NextRequest } from "next/server";
import { updateSession } from "@/lib/supabase";

export async function proxy(request: NextRequest) {
  return await updateSession(request);
}

/**
 * updateSession should run on _all_ requests outside of static assets
 * as we always want to update the user's session even for public pages (e.g., gitdot.io/public_repo)
 */
export const config = {
  matcher: [
    "/((?!_next/static|_next/image|favicon.ico|.*\\.(?:svg|png|jpg|jpeg|gif|webp)$).*)",
  ],
};
