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

export function timeAgo(date: Date) {
  const seconds = Math.floor((Date.now() - date.getTime()) / 1000);

  if (seconds < 60) return "just now";
  if (seconds < 3600) return `${Math.floor(seconds / 60)}m ago`;
  if (seconds < 86400) return `${Math.floor(seconds / 3600)}h ago`;
  if (seconds < 2592000) return `${Math.floor(seconds / 86400)}d ago`;
  if (seconds < 31536000) return `${Math.floor(seconds / 2592000)}mo ago`;
  return `${Math.floor(seconds / 31536000)}y ago`;
}

/**
 * Normalize date to start of day for grouping
 */
export function getDateKey(date: Date): string {
  const normalized = new Date(date);
  normalized.setHours(0, 0, 0, 0);
  return normalized.toISOString().split("T")[0]; // "YYYY-MM-DD"
}

/**
 * Group commits by date (most recent first)
 */
export function groupCommitsByDate<T extends { date: string }>(
  commits: T[],
): [string, T[]][] {
  const groups = new Map<string, T[]>();

  for (const commit of commits) {
    const date = new Date(commit.date);
    const key = getDateKey(date);

    if (!groups.has(key)) {
      groups.set(key, []);
    }
    groups.get(key)!.push(commit);
  }

  // Return sorted entries (most recent first)
  return Array.from(groups.entries()).sort((a, b) =>
    b[0].localeCompare(a[0]),
  );
}

/**
 * Format date header: "Today", "Yesterday", or "Jan 12"
 */
export function formatDateHeader(dateKey: string): string {
  const date = new Date(dateKey + "T00:00:00");
  const now = new Date();
  now.setHours(0, 0, 0, 0);

  const diffTime = now.getTime() - date.getTime();
  const diffDays = Math.floor(diffTime / (1000 * 60 * 60 * 24));

  if (diffDays === 0) return "Today";
  if (diffDays === 1) return "Yesterday";

  // Absolute format: "Jan 12" or "Dec 28, 2023" (with year if different)
  return date.toLocaleDateString("en-US", {
    month: "short",
    day: "numeric",
    year: date.getFullYear() !== now.getFullYear() ? "numeric" : undefined,
  });
}
