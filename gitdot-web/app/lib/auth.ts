import "server-only";

import {
  AuthTokensResource,
  type ExchangeGitHubCodeRequest,
  GitHubAuthRedirectResource,
  type LogoutRequest,
  type RefreshSessionRequest,
  type SendAuthEmailRequest,
  type VerifyAuthCodeRequest,
} from "gitdot-api";
import { cookies } from "next/headers";
import type { NextRequest } from "next/server";

const AUTH_URL = process.env.GITDOT_AUTH_URL ?? "http://localhost:8081";

// As we use SSR, setting cookies in the Rust server does not propagate them to the browser.
// Therefore, we set the cookies manually in the Next.js server. This is how Supabase does it as well.
const ACCESS_TOKEN_COOKIE = "gd_access_token";
const REFRESH_TOKEN_COOKIE = "gd_refresh_token";

// --- Cookie helpers ---

async function setTokenCookies(tokens: AuthTokensResource) {
  const store = await cookies();
  store.set(ACCESS_TOKEN_COOKIE, tokens.access_token, {
    httpOnly: true,
    secure: true,
    sameSite: "strict",
    path: "/",
    maxAge: tokens.access_token_expires_in,
  });
  store.set(REFRESH_TOKEN_COOKIE, tokens.refresh_token, {
    httpOnly: true,
    secure: true,
    sameSite: "strict",
    path: "/",
    maxAge: tokens.refresh_token_expires_in,
  });
}

async function clearTokenCookies() {
  const store = await cookies();
  store.delete(ACCESS_TOKEN_COOKIE);
  store.delete(REFRESH_TOKEN_COOKIE);
}

// --- User metadata ---

export interface UserMetadata {
  username: string;
  orgs: string[];
}

export async function getUserMetadata(): Promise<UserMetadata> {
  const session = await getSession();
  if (!session) return { username: "", orgs: [] };
  return session.user_metadata;
}

// --- Session ---

export async function getSession(): Promise<{
  access_token: string;
  user_metadata: UserMetadata;
} | null> {
  const store = await cookies();
  const token = store.get(ACCESS_TOKEN_COOKIE)?.value;
  if (!token) return null;

  try {
    const payload = JSON.parse(atob(token.split(".")[1]));
    if (payload.exp * 1000 < Date.now()) return null;
    return { access_token: token, user_metadata: payload.user_metadata };
  } catch {
    return null;
  }
}

export async function refreshSession(): Promise<{
  access_token: string;
} | null> {
  const store = await cookies();
  const refresh_token = store.get(REFRESH_TOKEN_COOKIE)?.value;
  if (!refresh_token) return null;

  const body: RefreshSessionRequest = { refresh_token };
  const res = await fetch(`${AUTH_URL}/auth/refresh`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(body),
  });

  if (!res.ok) {
    await clearTokenCookies();
    return null;
  }

  const tokens = AuthTokensResource.parse(await res.json());
  await setTokenCookies(tokens);
  return { access_token: tokens.access_token };
}

export async function updateSession(_request: NextRequest) {
  const session = await getSession();
  const access_token =
    session?.access_token ?? (await refreshSession())?.access_token;
  if (!access_token) return { user: null };

  const payload = JSON.parse(atob(access_token.split(".")[1]));
  return { user: payload };
}

// --- Email auth ---

export async function sendAuthEmail(email: string) {
  const body: SendAuthEmailRequest = { email };
  await fetch(`${AUTH_URL}/auth/email/send`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(body),
  });
}

export async function verifyAuthCode(code: string) {
  const body: VerifyAuthCodeRequest = { code };
  const res = await fetch(`${AUTH_URL}/auth/email/verify`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(body),
  });

  if (!res.ok) return null;

  const tokens = AuthTokensResource.parse(await res.json());
  await setTokenCookies(tokens);
  return { access_token: tokens.access_token, is_new: tokens.is_new };
}

// --- GitHub OAuth ---

export async function getGitHubRedirectUrl(): Promise<string | null> {
  const res = await fetch(`${AUTH_URL}/auth/github/redirect`);
  if (!res.ok) return null;
  const data = GitHubAuthRedirectResource.parse(await res.json());
  return data.authorize_url;
}

export async function exchangeGitHubCode(code: string, state: string) {
  const body: ExchangeGitHubCodeRequest = { code, state };
  const res = await fetch(`${AUTH_URL}/auth/github/exchange`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(body),
  });

  if (!res.ok) return null;

  const tokens = AuthTokensResource.parse(await res.json());
  await setTokenCookies(tokens);
  return { access_token: tokens.access_token, is_new: tokens.is_new };
}

// --- Logout ---

export async function logout() {
  const store = await cookies();
  const refresh_token = store.get(REFRESH_TOKEN_COOKIE)?.value;
  const access_token = store.get(ACCESS_TOKEN_COOKIE)?.value;

  if (refresh_token && access_token) {
    const body: LogoutRequest = { refresh_token };
    await fetch(`${AUTH_URL}/auth/logout`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        Authorization: `Bearer ${access_token}`,
      },
      body: JSON.stringify(body),
    });
  }

  await clearTokenCookies();
}
