import "server-only";

import type { ZodType } from "zod";
import { getSession } from "../supabase";

export const GITDOT_SERVER_URL =
  process.env.GITDOT_SERVER_URL || "http://localhost:8080";

export const NotFound = 404 as const;
export type NotFound = typeof NotFound;

export async function authFetch(
  url: string,
  options?: RequestInit,
): Promise<Response> {
  const session = await getSession();

  return fetch(url, {
    ...options,
    headers: {
      ...options?.headers,
      ...(session && { Authorization: `Bearer ${session.access_token}` }),
    },
  });
}

export async function authHead(
  url: string,
  options?: RequestInit,
): Promise<Response> {
  return await authFetch(url, {
    ...options,
    method: "HEAD",
  });
}

export async function authPost(
  url: string,
  request: unknown,
): Promise<Response> {
  return await authFetch(url, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(request),
  });
}

export async function authPatch(
  url: string,
  request: unknown,
): Promise<Response> {
  return await authFetch(url, {
    method: "PATCH",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(request),
  });
}

export async function handleResponse<T>(
  response: Response,
  schema: ZodType<T>,
): Promise<T | null> {
  if (!response.ok) {
    console.error(
      `${response.url} failed:`,
      response?.status,
      response?.statusText,
    );
    return null;
  }

  const data = await response.json();
  return schema.parse(data);
}
