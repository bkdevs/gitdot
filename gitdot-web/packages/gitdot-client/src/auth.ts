import "server-only";

import {
    type AddUserEmailRequest,
    type AuthorizeDeviceRequest,
    AuthTokensResource,
    type ExchangeGitHubCodeRequest,
    GitHubAuthRedirectResource,
    type LogoutRequest,
    type RefreshSessionRequest,
    type ResendVerificationCodeRequest,
    type SendAuthEmailRequest,
    SlackAccountResource,
    UserEmailResource,
    type VerifyAuthCodeRequest,
    type VerifyUserEmailRequest,
} from "gitdot-api";
import { cookies } from "next/headers";
import type { NextRequest, NextResponse } from "next/server";
import { authFetch, authPost, handleResponse } from "./util";

export const GITDOT_AUTH_SERVER_URL =
  process.env.GITDOT_AUTH_SERVER_URL ?? "http://localhost:8082";

// As we use SSR, setting cookies in the Rust server does not propagate them to the browser.
// Therefore, we set the cookies manually in the Next.js server. This is how Supabase does it as well.
const ACCESS_TOKEN_COOKIE = "gd_access_token";
const REFRESH_TOKEN_COOKIE = "gd_refresh_token";

// --- Cookie helpers ---

const sessionCookieOptions = (maxAge: number) => ({
  httpOnly: true,
  secure: true,
  sameSite: "lax" as const,
  path: "/",
  maxAge,
});

async function setTokenCookies(tokens: AuthTokensResource) {
  const store = await cookies();
  store.set(
    ACCESS_TOKEN_COOKIE,
    tokens.access_token,
    sessionCookieOptions(tokens.access_token_expires_in),
  );
  store.set(
    REFRESH_TOKEN_COOKIE,
    tokens.refresh_token,
    sessionCookieOptions(tokens.refresh_token_expires_in),
  );
}

async function clearTokenCookies() {
  const store = await cookies();
  store.delete(ACCESS_TOKEN_COOKIE);
  store.delete(REFRESH_TOKEN_COOKIE);
}

export function writeCookiesToResponse(
  response: NextResponse,
  tokens: AuthTokensResource,
) {
  response.cookies.set(
    ACCESS_TOKEN_COOKIE,
    tokens.access_token,
    sessionCookieOptions(tokens.access_token_expires_in),
  );
  response.cookies.set(
    REFRESH_TOKEN_COOKIE,
    tokens.refresh_token,
    sessionCookieOptions(tokens.refresh_token_expires_in),
  );
}

export function clearCookiesInResponse(response: NextResponse) {
  response.cookies.delete(ACCESS_TOKEN_COOKIE);
  response.cookies.delete(REFRESH_TOKEN_COOKIE);
}

// --- Session ---

export async function getSession(): Promise<{
  access_token: string;
} | null> {
  const store = await cookies();
  const token = store.get(ACCESS_TOKEN_COOKIE)?.value;
  if (!token) return null;

  try {
    const payload = JSON.parse(atob(token.split(".")[1]));
    if (payload.exp * 1000 < Date.now()) return null;
    return { access_token: token };
  } catch {
    return null;
  }
}

export type SessionUpdate = {
  user: unknown;
  tokens: AuthTokensResource | null;
};

export async function updateSession(
  request: NextRequest,
): Promise<SessionUpdate> {
  const session = await getSession();
  if (session) {
    const payload = JSON.parse(atob(session.access_token.split(".")[1]));
    return { user: payload, tokens: null };
  }

  const refresh_token = request.cookies.get(REFRESH_TOKEN_COOKIE)?.value;
  if (!refresh_token) return { user: null, tokens: null };

  const body: RefreshSessionRequest = { refresh_token };
  const res = await authFetch(`${GITDOT_AUTH_SERVER_URL}/auth/refresh`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(body),
  });

  if (!res.ok) return { user: null, tokens: null };

  const tokens = AuthTokensResource.parse(await res.json());
  const payload = JSON.parse(atob(tokens.access_token.split(".")[1]));
  return { user: payload, tokens };
}

// --- Email auth ---

export type AuthSignInResult = {
  is_new: boolean;
  username: string;
};

export async function sendAuthEmail(email: string) {
  const body: SendAuthEmailRequest = { email };
  await authFetch(`${GITDOT_AUTH_SERVER_URL}/auth/email/send`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(body),
  });
}

export async function verifyAuthCode(
  code: string,
): Promise<AuthSignInResult | null> {
  const body: VerifyAuthCodeRequest = { code };
  const res = await authFetch(`${GITDOT_AUTH_SERVER_URL}/auth/email/verify`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(body),
  });

  if (!res.ok) return null;

  const tokens = AuthTokensResource.parse(await res.json());
  await setTokenCookies(tokens);

  const payload = JSON.parse(atob(tokens.access_token.split(".")[1]));
  const username: string = payload.user_metadata?.username ?? "";

  return { is_new: tokens.is_new, username };
}

// --- Account management ---

export async function addUserEmail(email: string) {
  const body: AddUserEmailRequest = { email };
  const res = await authFetch(
    `${GITDOT_AUTH_SERVER_URL}/auth/account/add-email`,
    {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(body),
    },
  );
  return await handleResponse(res, UserEmailResource);
}

export async function verifyUserEmail(email: string, code: string) {
  const body: VerifyUserEmailRequest = { email, code };
  const res = await authFetch(
    `${GITDOT_AUTH_SERVER_URL}/auth/account/verify-email`,
    {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(body),
    },
  );
  return await handleResponse(res, UserEmailResource);
}

export async function resendUserEmailCode(email: string): Promise<boolean> {
  const body: ResendVerificationCodeRequest = { email };
  const res = await authFetch(
    `${GITDOT_AUTH_SERVER_URL}/auth/account/resend-code`,
    {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(body),
    },
  );
  return res.ok;
}

// --- GitHub OAuth ---

export async function getGitHubRedirectUrl(): Promise<string | null> {
  const res = await authFetch(`${GITDOT_AUTH_SERVER_URL}/auth/github/redirect`);
  if (!res.ok) return null;
  const data = GitHubAuthRedirectResource.parse(await res.json());
  return data.authorize_url;
}

export async function exchangeGitHubCode(
  code: string,
  state: string,
): Promise<AuthSignInResult | null> {
  const body: ExchangeGitHubCodeRequest = { code, state };
  const res = await authFetch(
    `${GITDOT_AUTH_SERVER_URL}/auth/github/exchange`,
    {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(body),
    },
  );

  if (!res.ok) return null;

  const tokens = AuthTokensResource.parse(await res.json());
  await setTokenCookies(tokens);

  const payload = JSON.parse(atob(tokens.access_token.split(".")[1]));
  const username: string = payload.user_metadata?.username ?? "";

  return { is_new: tokens.is_new, username };
}

// --- Device flow & Slack linking ---

export async function authorizeDevice(
  request: AuthorizeDeviceRequest,
): Promise<boolean> {
  const response = await authPost(
    `${GITDOT_AUTH_SERVER_URL}/auth/device/authorize`,
    request,
  );

  return response.ok;
}

export async function linkSlackAccount(
  state: string,
): Promise<SlackAccountResource | null> {
  const response = await authPost(`${GITDOT_AUTH_SERVER_URL}/auth/slack/link`, {
    state,
  });

  return await handleResponse(response, SlackAccountResource);
}

// --- Logout ---

export async function logout() {
  const store = await cookies();
  const refresh_token = store.get(REFRESH_TOKEN_COOKIE)?.value;
  const access_token = store.get(ACCESS_TOKEN_COOKIE)?.value;

  if (refresh_token && access_token) {
    const body: LogoutRequest = { refresh_token };
    await authFetch(`${GITDOT_AUTH_SERVER_URL}/auth/logout`, {
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
