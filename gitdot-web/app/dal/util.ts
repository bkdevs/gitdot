import "server-only";

import type { ZodType } from "zod";
import { getSession } from "@/lib/supabase";

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

export async function authDelete(
  url: string,
  options?: RequestInit,
): Promise<Response> {
  return await authFetch(url, {
    ...options,
    method: "DELETE",
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

export class ApiError extends Error {
  constructor(
    public readonly status: number,
    message: string,
  ) {
    super(message);
    this.name = "ApiError";
  }
}

export async function handleResponse<T>(
  response: Response,
  schema: ZodType<T>,
): Promise<T | null> {
  if (!response.ok) {
    let message = response.statusText;
    try {
      const body = await response.json();
      if (typeof body?.message === "string") message = body.message;
    } catch {
      // ignore parse failure, keep statusText
    }
    console.error(`${response.url} failed:`, response.status, message);
    throw new ApiError(response.status, message);
  }

  const data = await response.json();
  return schema.parse(data);
}

export async function handleEmptyResponse(response: Response): Promise<void> {
  if (!response.ok) {
    let message = response.statusText;
    try {
      const body = await response.json();
      if (typeof body?.message === "string") message = body.message;
    } catch {
      // ignore parse failure, keep statusText
    }
    console.error(`${response.url} failed:`, response.status, message);
    throw new ApiError(response.status, message);
  }
}
