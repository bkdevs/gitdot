import { type ClassValue, clsx } from "clsx";
import { twMerge } from "tailwind-merge";

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

export function validateRepoSlug(slug: string): boolean {
  return /^[a-zA-Z0-9_-]+$/.test(slug);
}

/**
 * helper to serialize objects that have non-string values into url parameter queries
 */
export function toQueryString(
  params: Record<string, string | number | boolean> | undefined,
): string {
  if (!params) {
    return "";
  }

  const stringParams = Object.fromEntries(
    Object.entries(params)
      .filter(([_, value]) => value !== undefined && value !== null)
      .map(([key, value]) => [key, String(value)]),
  );
  return new URLSearchParams(stringParams).toString();
}
