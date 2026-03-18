const MAX_AGE = 60 * 60 * 24; // 24h

export function repoCookieName(owner: string, repo: string) {
  return `gd_sha_${owner}_${repo}`;
}

// Client-side: call from a "use client" component
export async function setRepoCookie(
  owner: string,
  repo: string,
  sha: string,
): Promise<void> {
  const value = encodeURIComponent(
    JSON.stringify({ sha, at: new Date().toISOString() }),
  );

  const expires = Date.now() + MAX_AGE * 1000;
  await cookieStore.set({
    name: repoCookieName(owner, repo),
    value,
    path: "/",
    expires,
    sameSite: "lax",
  });
}

export function repoCookieHeaders(
  cookie: { sha: string; at: string } | null,
): Record<string, string> | undefined {
  if (!cookie) return undefined;
  return {
    "X-Gitdot-Client-Sha": cookie.sha,
    "X-Gitdot-Client-Timestamp": cookie.at,
  };
}

// Server-side: call from DAL / server components
export async function getRepoCookie(
  owner: string,
  repo: string,
): Promise<{ sha: string; at: string } | null> {
  const { cookies } = await import("next/headers");
  const raw = (await cookies()).get(repoCookieName(owner, repo))?.value;
  return raw ? JSON.parse(decodeURIComponent(raw)) : null;
}
