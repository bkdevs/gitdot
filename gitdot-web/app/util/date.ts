// ==================================
// timezone required date formatters
// use useTimezone to avoid date flicker and keep locality
// ==================================

export function formatDateIso(date: Date, tz: string): string {
  return new Intl.DateTimeFormat("en-CA", {
    timeZone: tz,
    year: "numeric",
    month: "2-digit",
    day: "2-digit",
  }).format(date);
}

export function formatDate(date: Date, tz: string): string {
  return new Intl.DateTimeFormat("en-US", {
    timeZone: tz,
    month: "short",
    day: "numeric",
    year: "numeric",
  }).format(date);
}

export function formatTime(date: Date, tz: string): string {
  return new Intl.DateTimeFormat("en-US", {
    timeZone: tz,
    hour: "numeric",
    minute: "2-digit",
    hour12: true,
  }).format(date);
}

export function formatDateTime(date: Date, tz: string): string {
  const parts = new Intl.DateTimeFormat("en-US", {
    timeZone: tz,
    month: "short",
    day: "numeric",
    year: "numeric",
    hour: "numeric",
    minute: "2-digit",
    second: "2-digit",
    hour12: true,
  }).formatToParts(date);
  const get = (t: string) => parts.find((p) => p.type === t)?.value ?? "";
  return `${get("month")} ${get("day")}, ${get("year")} ${get("hour")}:${get("minute")}:${get("second")} ${get("dayPeriod")}`;
}

export function subtractMonths(date: Date, months: number, tz: string): string {
  const [y, m] = formatDateIso(date, tz).split("-").map(Number);
  const total = y * 12 + (m - 1) - months;
  const targetY = Math.floor(total / 12);
  const targetM = total - targetY * 12 + 1;
  return `${targetY}-${String(targetM).padStart(2, "0")}-01`;
}

// ============================
// timezone invariant utilities
// ============================
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

export function addDays(date: Date, days: number): Date {
  const d = new Date(date);
  d.setDate(d.getDate() + days);
  return d;
}

export function subtractDays(date: Date, days: number): Date {
  return addDays(date, -days);
}

export function timeAgo(date: Date) {
  const seconds = Math.floor((Date.now() - date.getTime()) / 1000);

  if (seconds < 60) return "just now";
  if (seconds < 3600) return `${Math.floor(seconds / 60)}m ago`;
  if (seconds < 86400) return `${Math.floor(seconds / 3600)}h ago`;

  const days = Math.floor(seconds / 86400);
  const remHours = Math.floor((seconds % 86400) / 3600);
  if (seconds < 3 * 86400) {
    return remHours > 0 ? `${days}d ${remHours}h ago` : `${days}d ago`;
  }

  if (seconds < 2592000) return `${days}d ago`;
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

export function dateInRange(
  date: string,
  start: string | null,
  end: string | null,
): boolean {
  if (!start || !end) return false;
  const lo = start <= end ? start : end;
  const hi = start <= end ? end : start;
  return date >= lo && date <= hi;
}

/**
 * Format a YYYY-MM-DD calendar string as "Jan 12, 2025". Tz-stable because the
 * Y/M/D round-trips through local midnight; no tz argument needed.
 */
export function formatCalendarDate(yyyymmdd: string): string {
  const [y, m, d] = yyyymmdd.split("-").map(Number);
  return new Date(y, m - 1, d).toLocaleDateString("en-US", {
    month: "short",
    day: "numeric",
    year: "numeric",
  });
}
