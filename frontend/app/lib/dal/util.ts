import "server-only";

import type { ZodType } from "zod";
import { getSession } from "../supabase";

export const API_BASE_URL = process.env.API_BASE_URL || "http://localhost:8080";

export async function authFetch(
  url: string,
  options?: RequestInit,
): Promise<Response> {
  const session = await getSession();
  if (!session) throw new Error("User not authenticated!");

  return fetch(url, {
    ...options,
    headers: {
      ...options?.headers,
      Authorization: `Bearer ${session.access_token}`,
    },
  });
}

export async function authPost(url: string, request: any): Promise<Response> {
  return await authFetch(url, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(request),
  });
}

export async function authPatch(url: string, request: any): Promise<Response> {
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
