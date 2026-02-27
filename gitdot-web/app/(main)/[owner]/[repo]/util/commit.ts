import type { RepositoryCommitResource } from "gitdot-api-ts";

/**
 * Group commits by date (most recent first)
 */
export function groupCommitsByDate(
  commits: RepositoryCommitResource[],
): [string, RepositoryCommitResource[]][] {
  const groups = new Map<string, RepositoryCommitResource[]>();

  for (const commit of commits) {
    const date = new Date(commit.date);
    const normalized = new Date(date);
    normalized.setHours(0, 0, 0, 0);
    const key = normalized.toISOString().split("T")[0];

    if (!groups.has(key)) {
      groups.set(key, []);
    }
    groups.get(key)?.push(commit);
  }

  return Array.from(groups.entries()).sort((a, b) => b[0].localeCompare(a[0]));
}
