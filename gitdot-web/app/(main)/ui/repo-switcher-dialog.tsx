"use client";

import { VisuallyHidden } from "@radix-ui/react-visually-hidden";
import type { UserOrganizationResource, UserResource } from "gitdot-api";
import { useRouter } from "next/navigation";
import { useEffect, useMemo, useRef, useState } from "react";
import { useShortcuts } from "@/(main)/context/shortcuts";
import { useUserContext } from "@/(main)/context/user";
import {
  listOrganizationRepositoriesAction,
  listUserRepositoriesAction,
  listUserStarredRepositoriesAction,
} from "@/actions";
import { Dialog, DialogContent, DialogTitle } from "@/ui/dialog";

type RepoListItem = {
  owner: string;
  name: string;
  type: "user" | "org" | "star" | "recent";
  visitedAt?: number;
};

const MAX_RECENTS = 9;

export function RepoSwitcherDialog() {
  const { user, memberships } = useUserContext();
  if (!user || memberships === undefined) return null;

  return (
    <RepoSwitcherDialogInner user={user} memberships={memberships ?? []} />
  );
}

function RepoSwitcherDialogInner({
  user,
  memberships,
}: {
  user: UserResource;
  memberships: UserOrganizationResource[];
}) {
  const router = useRouter();
  const [open, setOpen] = useState(false);
  const [repositories, setRepositories] = useState<RepoListItem[] | null>(null);
  const [recentRepositories, setRecentRepositories] = useState<RepoListItem[]>(
    [],
  );

  const [query, setQuery] = useState("");
  const [selectedIdx, setSelectedIdx] = useState(0);
  const [flashIdx, setFlashIdx] = useState<number | null>(null);
  const listRef = useRef<HTMLDivElement>(null);

  useShortcuts([
    {
      name: "Repos",
      description: "Open repositories",
      keys: ["r"],
      execute: () => setOpen(true),
    },
  ]);

  useEffect(() => {
    const handle = () => setOpen(true);
    window.addEventListener("openRepos", handle);
    return () => window.removeEventListener("openRepos", handle);
  }, []);

  useEffect(() => {
    Promise.all([
      listUserRepositoriesAction(user.name),
      Promise.all(
        memberships.map((m) => listOrganizationRepositoriesAction(m.name)),
      ),
      listUserStarredRepositoriesAction(user.name),
    ]).then(([userRepos, orgReposByOrg, starredRepos]) => {
      const items: RepoListItem[] = [];
      const seen = new Set<string>();
      const pushRepos = (
        list: { owner: string; name: string }[],
        type: RepoListItem["type"],
      ) => {
        for (const r of list) {
          const key = `${r.owner}/${r.name}`;
          if (seen.has(key)) continue;
          seen.add(key);
          items.push({ owner: r.owner, name: r.name, type });
        }
      };

      pushRepos(userRepos, "user");
      for (const orgRepos of orgReposByOrg) {
        pushRepos(orgRepos, "org");
      }
      pushRepos(starredRepos, "star");
      setRepositories(items);
    });
  }, [user, memberships]);

  useEffect(() => {
    if (open) {
      setRecentRepositories(readRecentRepos());
    } else {
      setQuery("");
      setSelectedIdx(0);
      setFlashIdx(null);
    }
  }, [open]);

  const filteredRepositories = useMemo(() => {
    if (!repositories) return [];
    const q = query.trim().toLowerCase();
    if (!q) return repositories;
    return repositories.filter((repo) =>
      `${repo.owner}/${repo.name}`.toLowerCase().includes(q),
    );
  }, [repositories, query]);

  useEffect(() => {
    if (selectedIdx >= filteredRepositories.length) setSelectedIdx(0);
  }, [filteredRepositories.length, selectedIdx]);

  useEffect(() => {
    const el = listRef.current?.children[selectedIdx] as
      | HTMLElement
      | undefined;
    el?.scrollIntoView({ block: "nearest" });
  }, [selectedIdx]);

  const openRepo = (repo: { owner: string; name: string }) => {
    setOpen(false);
    router.push(`/${repo.owner}/${repo.name}`);
  };

  const navigateToRecentRepo = (idx: number) => {
    const target = recentRepositories[idx];
    if (!target) return;
    setFlashIdx(idx);
    setTimeout(() => openRepo(target), 200);
  };

  return (
    <Dialog open={open} onOpenChange={setOpen}>
      <DialogContent
        className="max-w-180 min-w-180 border-black rounded-xs shadow-2xl top-[42%] p-0"
        animations={false}
        showOverlay={false}
      >
        <VisuallyHidden>
          <DialogTitle>Repositories</DialogTitle>
        </VisuallyHidden>
        <div className="flex">
          <div className="flex-1 flex flex-col border-r border-border min-w-0">
            <input
              autoFocus
              type="text"
              value={query}
              onChange={(e) => setQuery(e.target.value)}
              onKeyDown={(e) => {
                if (e.key === "ArrowDown" || (e.ctrlKey && e.key === "n")) {
                  e.preventDefault();
                  setSelectedIdx((i) =>
                    Math.min(i + 1, filteredRepositories.length - 1),
                  );
                } else if (
                  e.key === "ArrowUp" ||
                  (e.ctrlKey && e.key === "p")
                ) {
                  e.preventDefault();
                  setSelectedIdx((i) => Math.max(i - 1, 0));
                } else if (e.key === "Enter") {
                  e.preventDefault();
                  const selected = filteredRepositories[selectedIdx];
                  if (selected) openRepo(selected);
                } else if (
                  !e.metaKey &&
                  !e.ctrlKey &&
                  !e.altKey &&
                  /^[0-9]$/.test(e.key)
                ) {
                  e.preventDefault();
                  const idx = e.key === "0" ? 9 : Number(e.key) - 1;
                  navigateToRecentRepo(idx);
                }
              }}
              placeholder="search..."
              className="w-full h-9 px-2 text-sm font-mono bg-background outline-none border-b border-border"
            />
            <div ref={listRef} className="flex flex-col h-80 overflow-y-auto">
              {repositories === null ? (
                <div className="px-2 py-1 text-sm font-mono text-muted-foreground">
                  loading...
                </div>
              ) : filteredRepositories.length === 0 ? (
                <div className="px-2 py-1 text-sm font-mono text-muted-foreground">
                  no repos found
                </div>
              ) : (
                filteredRepositories.map((repo, idx) => (
                  <button
                    key={`${repo.owner}/${repo.name}`}
                    type="button"
                    onMouseEnter={() => setSelectedIdx(idx)}
                    onClick={() => openRepo(repo)}
                    className={`flex items-baseline justify-between px-2 py-0.5 text-left outline-none ring-0 focus:outline-none focus:ring-0 focus-visible:outline-none focus-visible:ring-0 ${
                      idx === selectedIdx
                        ? "bg-accent text-accent-foreground"
                        : ""
                    }`}
                  >
                    <span className="text-sm font-mono">
                      {repo.owner}/{repo.name}
                    </span>
                    <span className="text-xs font-mono text-muted-foreground">
                      {repo.type}
                    </span>
                  </button>
                ))
              )}
            </div>
          </div>
          <div className="w-56 shrink-0 flex flex-col">
            <div className="px-2 pt-2 pb-1 flex items-center text-sm font-mono text-muted-foreground">
              Recent repositories
            </div>
            <div className="flex flex-col flex-1 overflow-y-auto min-h-0">
              {recentRepositories.length === 0 ? (
                <div className="px-2 py-1 text-sm font-mono text-muted-foreground">
                  (none)
                </div>
              ) : (
                recentRepositories.map((repo, idx) => {
                  const flashed = idx === flashIdx;
                  return (
                    <button
                      key={`${repo.owner}/${repo.name}`}
                      type="button"
                      onClick={() => openRepo(repo)}
                      className="group flex items-baseline gap-1 px-2 text-left cursor-pointer outline-none ring-0 focus:outline-none focus:ring-0 focus-visible:outline-none focus-visible:ring-0"
                    >
                      <span
                        className={`text-xs font-mono transition-colors duration-200 shrink-0 ${
                          flashed
                            ? "text-foreground"
                            : "text-muted-foreground group-hover:text-foreground"
                        }`}
                      >
                        {idx + 1}.
                      </span>
                      <span
                        className={`min-w-0 flex-1 text-sm font-mono truncate underline transition-colors duration-200 ${
                          flashed
                            ? "decoration-current"
                            : "decoration-transparent group-hover:decoration-current"
                        }`}
                      >
                        {repo.owner}/{repo.name}
                      </span>
                    </button>
                  );
                })
              )}
            </div>
            <div className="px-2 py-1 text-xs font-mono text-muted-foreground">
              Press 1-9 to navigate
            </div>
          </div>
        </div>
      </DialogContent>
    </Dialog>
  );
}

function readRecentRepos(): RepoListItem[] {
  if (typeof window === "undefined") return [];
  try {
    const raw = localStorage.getItem("gitdot_recent_repos");
    if (!raw) return [];
    const parsed = JSON.parse(raw);
    if (!Array.isArray(parsed)) return [];

    return parsed
      .filter(
        (r): r is { owner: string; name: string; visitedAt: number } =>
          typeof r === "object" &&
          r !== null &&
          typeof r.owner === "string" &&
          typeof r.name === "string" &&
          typeof r.visitedAt === "number",
      )
      .slice(0, MAX_RECENTS)
      .map((r) => ({ ...r, type: "recent" as const }));
  } catch {
    return [];
  }
}
