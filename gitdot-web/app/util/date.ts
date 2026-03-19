const MS_PER_DAY = 86400000;

export function addDays(date: Date, days: number): Date {
  return new Date(date.getTime() + days * MS_PER_DAY);
}

export function dateOnly(value: string): Date;
export function dateOnly(value: Date): Date;
export function dateOnly(value: string | Date): Date {
  if (typeof value === "string") {
    const d = new Date(value.slice(0, 10) + "T00:00:00");
    return d;
  }
  const d = new Date(value);
  d.setHours(0, 0, 0, 0);
  return d;
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

export function timeAgoFull(date: Date) {
  const seconds = Math.floor((Date.now() - date.getTime()) / 1000);
  if (seconds < 60) return "just now";

  const p = (n: number, unit: string) => `${n} ${unit}${n === 1 ? "" : "s"}`;

  const minutes = Math.floor(seconds / 60);
  if (seconds < 3600) return `${p(minutes, "minute")} ago`;

  const hours = Math.floor(seconds / 3600);
  if (seconds < 86400) return `${p(hours, "hour")} ago`;

  const days = Math.floor(seconds / 86400);
  const remHours = Math.floor((seconds % 86400) / 3600);
  if (seconds < 2592000) {
    return remHours > 0
      ? `${p(days, "day")}, ${p(remHours, "hour")} ago`
      : `${p(days, "day")} ago`;
  }

  const months = Math.floor(seconds / 2592000);
  const remDays = Math.floor((seconds % 2592000) / 86400);
  if (seconds < 31536000) {
    return remDays > 0
      ? `${p(months, "month")}, ${p(remDays, "day")} ago`
      : `${p(months, "month")} ago`;
  }

  const years = Math.floor(seconds / 31536000);
  const remMonths = Math.floor((seconds % 31536000) / 2592000);
  return remMonths > 0
    ? `${p(years, "year")}, ${p(remMonths, "month")} ago`
    : `${p(years, "year")} ago`;
}

/**
 * Format date header: "Today", "Yesterday", or "Jan 12"
 */
export function formatDateKey(dateKey: string): string {
  const date = new Date(`${dateKey}T00:00:00`);
  const now = new Date();
  now.setHours(0, 0, 0, 0);

  const diffTime = now.getTime() - date.getTime();
  const diffDays = Math.floor(diffTime / (1000 * 60 * 60 * 24));

  if (diffDays === 0) return "Today";
  if (diffDays === 1) return "Yesterday";

  return date.toLocaleDateString("en-US", {
    month: "short",
    day: "numeric",
    year: date.getFullYear() !== now.getFullYear() ? "numeric" : undefined,
  });
}

/**
 * Format date as "Jan 12, 2025"
 */
export function formatDate(date: Date): string {
  return date.toLocaleDateString("en-US", {
    month: "short",
    day: "numeric",
    year: "numeric",
  });
}

/**
 * Format time from date: "2:30 PM"
 */
export function formatTime(date: Date): string {
  return date.toLocaleTimeString("en-US", {
    hour: "numeric",
    minute: "2-digit",
    hour12: true,
  });
}

/**
 * For use in formal settings
 */
export function formatDateTime(date: Date): string {
  const datePart = date.toDateString(); // "Wed Jan 14 2026"
  const monthDay = datePart.slice(4, 10); // "Jan 14"
  const year = date.getFullYear();

  let hours = date.getHours();
  const minutes = String(date.getMinutes()).padStart(2, "0");
  const seconds = String(date.getSeconds()).padStart(2, "0");

  const ampm = hours >= 12 ? "PM" : "AM";
  hours = hours % 12 || 12; // Convert to 12-hour format, 0 becomes 12

  return `${monthDay}, ${year} ${hours}:${minutes}:${seconds} ${ampm}`;
}

export function formatDuration(ms: number): string {
  const totalSeconds = Math.floor(ms / 1000);
  if (totalSeconds < 60) return `${totalSeconds}s`;
  const minutes = Math.floor(totalSeconds / 60);
  const seconds = totalSeconds % 60;
  if (minutes < 60)
    return seconds > 0 ? `${minutes}m ${seconds}s` : `${minutes}m`;
  const hours = Math.floor(minutes / 60);
  const remainingMinutes = minutes % 60;
  return remainingMinutes > 0 ? `${hours}h ${remainingMinutes}m` : `${hours}h`;
}
