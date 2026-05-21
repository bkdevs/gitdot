"use client";

import { VisuallyHidden } from "@radix-ui/react-visually-hidden";
import type {
  OrganizationMemberResource,
  RepositoryResource,
  UserResource,
} from "gitdot-api";
import { useRouter } from "next/navigation";
import { useEffect, useMemo, useRef, useState } from "react";
import { useShortcuts } from "@/(main)/provider/shortcuts";
import { useUserContext } from "@/(main)/provider/user";
import {
  listOrganizationRepositoriesAction,
  listUserRepositoriesAction,
} from "@/actions";
import { Dialog, DialogContent, DialogTitle } from "@/ui/dialog";

export function ReposDialog() {
  const { user, memberships } = useUserContext();
  if (!user || memberships === undefined) return null;

  return <ReposDialogInner user={user} memberships={memberships ?? []} />;
}

function ReposDialogInner({
  user,
  memberships,
}: {
  user: UserResource;
  memberships: OrganizationMemberResource[];
}) {
  const router = useRouter();
  const [open, setOpen] = useState(false);
  const [query, setQuery] = useState("");
  const [selectedIdx, setSelectedIdx] = useState(0);
  const [repositories, setRepositories] = useState<RepositoryResource[] | null>(
    null,
  );
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
      ...memberships.map((m) => listOrganizationRepositoriesAction(m.org_name)),
    ]).then((results) => setRepositories(results.flat()));
  }, [user, memberships]);

  useEffect(() => {
    if (!open) {
      setQuery("");
      setSelectedIdx(0);
    }
  }, [open]);

  const filtered = useMemo(() => {
    if (!repositories) return [];
    const q = query.trim().toLowerCase();
    if (!q) return repositories;
    return repositories.filter((repo) =>
      `${repo.owner}/${repo.name}`.toLowerCase().includes(q),
    );
  }, [repositories, query]);

  useEffect(() => {
    setSelectedIdx(0);
  }, []);

  useEffect(() => {
    if (selectedIdx >= filtered.length) setSelectedIdx(0);
  }, [filtered.length, selectedIdx]);

  useEffect(() => {
    const el = listRef.current?.children[selectedIdx] as
      | HTMLElement
      | undefined;
    el?.scrollIntoView({ block: "nearest" });
  }, [selectedIdx]);

  const openRepo = (repo: RepositoryResource) => {
    setOpen(false);
    router.push(`/${repo.owner}/${repo.name}`);
  };

  return (
    <Dialog open={open} onOpenChange={setOpen}>
      <DialogContent
        className="max-w-xl min-w-xl border-black top-[40%] p-0 overflow-hidden"
        animations={false}
        showOverlay={false}
      >
        <VisuallyHidden>
          <DialogTitle>Repositories</DialogTitle>
        </VisuallyHidden>
        <div className="flex flex-col">
          <input
            autoFocus
            type="text"
            value={query}
            onChange={(e) => setQuery(e.target.value)}
            onKeyDown={(e) => {
              if (e.key === "ArrowDown" || (e.ctrlKey && e.key === "n")) {
                e.preventDefault();
                setSelectedIdx((i) => Math.min(i + 1, filtered.length - 1));
              } else if (e.key === "ArrowUp" || (e.ctrlKey && e.key === "p")) {
                e.preventDefault();
                setSelectedIdx((i) => Math.max(i - 1, 0));
              } else if (e.key === "Enter") {
                e.preventDefault();
                const selected = filtered[selectedIdx];
                if (selected) openRepo(selected);
              }
            }}
            placeholder="search repositories..."
            className="w-full h-9 px-2 text-sm font-mono bg-background outline-none border-b border-border"
          />
          <div ref={listRef} className="flex flex-col h-64 overflow-y-auto">
            {repositories === null ? (
              <div className="px-2 py-1 text-sm font-mono text-muted-foreground">
                loading...
              </div>
            ) : filtered.length === 0 ? (
              <div className="px-2 py-1 text-sm font-mono text-muted-foreground">
                no repos found
              </div>
            ) : (
              filtered.map((repo, idx) => (
                <button
                  key={`${repo.owner}/${repo.name}`}
                  type="button"
                  onMouseEnter={() => setSelectedIdx(idx)}
                  onClick={() => openRepo(repo)}
                  className={`flex flex-col gap-0.5 px-2 py-1 text-left border-b border-border cursor-pointer outline-none ring-0 focus:outline-none focus:ring-0 focus-visible:outline-none focus-visible:ring-0 ${
                    idx === selectedIdx
                      ? "bg-accent text-accent-foreground"
                      : ""
                  }`}
                >
                  <span className="text-sm font-mono">
                    {repo.owner}/{repo.name}
                  </span>
                </button>
              ))
            )}
          </div>
        </div>
      </DialogContent>
    </Dialog>
  );
}
