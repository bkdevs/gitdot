import {
  clearCookiesInResponse,
  updateSession,
  writeCookiesToResponse,
} from "gitdot-client";
import { type NextRequest, NextResponse } from "next/server";

const IS_BETA = process.env.NEXT_PUBLIC_GITDOT_BETA === "true";
const BETA_PATHS = new Set(["questions", "reviews", "builds"]);

export async function proxy(request: NextRequest) {
  const { user, tokens } = await updateSession(request);

  let response: NextResponse;

  const pathname = request.nextUrl.pathname;
  const segments = pathname.split("/").filter(Boolean);
  if (user && (pathname === "/login" || pathname === "/signup")) {
    response = NextResponse.redirect(new URL("/home", request.nextUrl));
  } else if (!user && pathname === "/oauth/device") {
    response = NextResponse.redirect(
      new URL("/login?redirect=/oauth/device", request.nextUrl),
    );
  } else if (!IS_BETA && segments.length >= 3 && BETA_PATHS.has(segments[2])) {
    response = new NextResponse(null, { status: 404 });
  } else {
    response = NextResponse.next({ request });
  }

  if (tokens) {
    writeCookiesToResponse(response, tokens);
  } else if (!user) {
    clearCookiesInResponse(response);
  }
  return response;
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
