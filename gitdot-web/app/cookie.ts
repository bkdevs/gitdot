const MAX_AGE = 60 * 60 * 24; // 24h

export function repoCookieName(owner: string, repo: string) {
  return `gd_sha_${owner}_${repo}`;
}

// Client-side: call from a "use client" component
export function setRepoCookie(owner: string, repo: string, sha: string): void {
  const value = encodeURIComponent(
    JSON.stringify({ sha, at: new Date().toISOString() }),
  );
  document.cookie = `${repoCookieName(owner, repo)}=${value}; path=/; max-age=${MAX_AGE}; SameSite=Lax`;
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
