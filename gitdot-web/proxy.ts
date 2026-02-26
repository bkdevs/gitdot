import { type NextRequest, NextResponse } from "next/server";
import { updateSession } from "@/lib/supabase";

const ORG_SLUGS = ["gitdot"];

export async function proxy(request: NextRequest) {
  const { user, response } = await updateSession(request);

  const pathname = request.nextUrl.pathname;
  if (user && (pathname === "/login" || pathname === "/signup")) {
    return NextResponse.redirect(new URL("/home", request.nextUrl));
  } else if (!user && pathname === "/oauth/device") {
    return NextResponse.redirect(
      new URL("/login?redirect=/oauth/device", request.nextUrl),
    );
  }
  const segments = pathname.split("/").filter(Boolean);
  const [firstSegment, secondSegment] = segments;

  if (ORG_SLUGS.includes(firstSegment)) {
    const isSpecialPath = secondSegment === "settings";
    const isAlreadyDuplicated = secondSegment === firstSegment;

    // TODO: there is ambiguity between a non-default repo (e.g., gitdot/gitdot-infra and files, gitdot/README.md)
    if (!isSpecialPath && !isAlreadyDuplicated) {
      const pathParts = [firstSegment, firstSegment, ...segments.slice(1)];
      const newPathname = `/${pathParts.join("/")}`;
      return NextResponse.rewrite(
        new URL(newPathname, request.nextUrl),
        response,
      );
    }
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
