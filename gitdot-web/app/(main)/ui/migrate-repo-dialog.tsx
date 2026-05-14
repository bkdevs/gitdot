"use client";

import { VisuallyHidden } from "@radix-ui/react-visually-hidden";
import type { GitHubRepositoryResource } from "gitdot-api";
import { ChevronDown } from "lucide-react";
import Image from "next/image";
import { useEffect, useMemo, useState } from "react";
import { OrgImage } from "@/(main)/[owner]/ui/org/org-image";
import { UserImage } from "@/(main)/[owner]/ui/user/user-image";
import { useUserContext } from "@/(main)/context/user";
import { listInstallationRepositoriesAction } from "@/actions";
import { Dialog, DialogContent, DialogTitle } from "@/ui/dialog";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/ui/dropdown-menu";
import { cn } from "@/util";
import { timeAgo } from "@/util/date";

type MigrationType = "read-only" | "read-write";

const TYPE_OPTIONS: { value: MigrationType; label: string }[] = [
  { value: "read-only", label: "Read-only" },
  { value: "read-write", label: "Read-write" },
];

export function MigrateRepoDialog() {
  const { user, memberships, installations } = useUserContext();

  const [open, setOpen] = useState(false);
  const [githubAccountId, setGithubAccountId] = useState<number | null>(null);
  const [gitdotAccountName, setGitdotAccount] = useState("");
  const [repos, setRepos] = useState<GitHubRepositoryResource[] | null>(null);
  const [selectedRepos, setSelectedRepos] = useState<Set<string>>(new Set());
  const [migrationType, setMigrationType] =
    useState<MigrationType>("read-only");

  const selectedMembership = memberships?.find(
    (m) => m.org_name === gitdotAccountName,
  );

  const githubAccount =
    githubAccountId !== null
      ? installations?.find((i) => i.installation_id === githubAccountId)
      : undefined;

  useEffect(() => {
    if (!open) {
      setSelectedRepos(new Set());
      setMigrationType("read-only");
    }
  }, [open]);

  useEffect(() => {
    const handle = () => setOpen(true);
    window.addEventListener("openMigrateRepo", handle);
    return () => window.removeEventListener("openMigrateRepo", handle);
  }, []);

  useEffect(() => {
    if (!user) return;
    if (gitdotAccountName) return;
    setGitdotAccount(user.name);
  }, [user, gitdotAccountName]);

  useEffect(() => {
    if (!installations || installations.length === 0) return;
    if (githubAccountId !== null) return;
    setGithubAccountId(installations[0].installation_id);
  }, [installations, githubAccountId]);

  useEffect(() => {
    if (githubAccountId === null) return;
    const id = githubAccountId;
    setRepos(null);
    let cancelled = false;
    listInstallationRepositoriesAction(id).then((result) => {
      if (cancelled) return;
      setRepos(result);
    });
    return () => {
      cancelled = true;
    };
  }, [githubAccountId]);

  const sortedRepos = useMemo(() => {
    if (!repos) return repos;
    return [...repos].sort((a, b) => {
      const aTime = a.pushed_at ? new Date(a.pushed_at).getTime() : 0;
      const bTime = b.pushed_at ? new Date(b.pushed_at).getTime() : 0;
      return bTime - aTime;
    });
  }, [repos]);

  const toggleRepo = (name: string) => {
    setSelectedRepos((prev) => {
      const next = new Set(prev);
      if (next.has(name)) {
        next.delete(name);
      } else {
        next.add(name);
      }
      return next;
    });
  };

  const isValid = selectedRepos.size > 0;

  return (
    <Dialog open={open} onOpenChange={setOpen}>
      <DialogContent
        className="max-w-3xl min-w-3xl border-black rounded-xs shadow-2xl top-[45%] p-0 overflow-hidden"
        animations={true}
        showOverlay={true}
      >
        <VisuallyHidden>
          <DialogTitle>Migrate repositories</DialogTitle>
        </VisuallyHidden>
        <div className="relative">
          <div className="flex">
            <div className="flex flex-col w-2/3 border-r border-b border-border">
              <div className="flex flex-col gap-1.5 px-2 py-1.5 border-b border-border">
                <div className="flex items-center justify-between text-xs">
                  <div className="flex items-center gap-2">
                    <span className="text-muted-foreground w-8 shrink-0">
                      From:
                    </span>
                    <DropdownMenu>
                      <DropdownMenuTrigger className="flex items-center gap-1.5 hover:text-muted-foreground transition-colors cursor-pointer">
                        <Image
                          src="/github-logo.svg"
                          alt=""
                          width={13}
                          height={13}
                        />
                        {installations === undefined
                          ? "loading..."
                          : githubAccount
                            ? `github.com/${githubAccount.github_login}`
                            : "select"}
                        <ChevronDown className="size-3" />
                      </DropdownMenuTrigger>
                      <DropdownMenuContent align="start" className="min-w-32">
                        {(installations ?? []).map((installation) => (
                          <DropdownMenuItem
                            key={installation.id}
                            className="text-xs"
                            onClick={() =>
                              setGithubAccountId(installation.installation_id)
                            }
                          >
                            <Image
                              src="/github-logo.svg"
                              alt=""
                              width={13}
                              height={13}
                            />
                            github.com/{installation.github_login}
                          </DropdownMenuItem>
                        ))}
                      </DropdownMenuContent>
                    </DropdownMenu>
                  </div>
                  <button
                    type="button"
                    className="underline hover:text-muted-foreground transition-colors cursor-pointer"
                  >
                    Install GitHub app
                  </button>
                </div>
                <div className="flex items-center gap-2 text-xs">
                  <span className="text-muted-foreground w-8 shrink-0">
                    To:
                  </span>
                  <DropdownMenu>
                    <DropdownMenuTrigger className="flex items-center gap-1.5 hover:text-muted-foreground transition-colors cursor-pointer">
                      {selectedMembership ? (
                        <OrgImage
                          orgId={selectedMembership.organization_id}
                          px={14}
                        />
                      ) : (
                        <UserImage userId={user?.id} px={14} />
                      )}
                      {gitdotAccountName
                        ? `gitdot.io/${gitdotAccountName}`
                        : "select"}
                      <ChevronDown className="size-3" />
                    </DropdownMenuTrigger>
                    <DropdownMenuContent align="start" className="min-w-32">
                      {user && (
                        <DropdownMenuItem
                          className="text-xs"
                          onClick={() => setGitdotAccount(user.name)}
                        >
                          <UserImage userId={user.id} px={14} />
                          gitdot.io/{user.name}
                        </DropdownMenuItem>
                      )}
                      {memberships?.map((m) => (
                        <DropdownMenuItem
                          key={m.organization_id}
                          className="text-xs"
                          onClick={() => setGitdotAccount(m.org_name)}
                        >
                          <OrgImage orgId={m.organization_id} px={14} />
                          gitdot.io/{m.org_name}
                        </DropdownMenuItem>
                      ))}
                    </DropdownMenuContent>
                  </DropdownMenu>
                </div>
              </div>
              <div className="flex flex-col h-84 overflow-y-auto scrollbar-thin">
                {sortedRepos === null ? (
                  <div className="px-2 py-1.5 text-xs font-mono text-muted-foreground">
                    loading...
                  </div>
                ) : sortedRepos.length === 0 ? (
                  <div className="px-2 py-1.5 text-xs text-muted-foreground">
                    No repositories found.
                  </div>
                ) : (
                  sortedRepos.map((repo) => {
                    const checked = selectedRepos.has(repo.full_name);
                    return (
                      <button
                        key={repo.id}
                        type="button"
                        onClick={() => toggleRepo(repo.full_name)}
                        className="flex items-start gap-2 px-2 py-1.5 text-xs text-left border-b border-border/50 last:border-b-0 hover:bg-accent/50 transition-colors duration-150"
                      >
                        <div
                          className={cn(
                            "mt-[3px] shrink-0 w-3 h-3 rounded-xs border border-border transition-colors duration-150",
                            checked ? "bg-foreground" : "bg-background",
                          )}
                        />
                        <div className="flex items-end justify-between gap-2 min-w-0 flex-1">
                          <div className="flex flex-col min-w-0">
                            <div className="flex items-center gap-1.5 min-w-0">
                              <span className="truncate">{repo.full_name}</span>
                              {checked && gitdotAccountName && (
                                <span className="text-muted-foreground truncate">
                                  → {gitdotAccountName}/{repo.name}
                                </span>
                              )}
                            </div>
                            <span
                              className={cn(
                                "truncate text-muted-foreground",
                                !repo.description && "italic opacity-60",
                              )}
                            >
                              {repo.description ?? "no description"}
                            </span>
                          </div>
                          {repo.pushed_at && (
                            <span className="shrink-0 text-xs font-mono text-muted-foreground">
                              last commit {timeAgo(new Date(repo.pushed_at))}
                            </span>
                          )}
                        </div>
                      </button>
                    );
                  })
                )}
              </div>
            </div>
            <div className="flex flex-col justify-between w-1/3 border-b border-border">
              <div className="p-2">
                <h2 className="text-sm font-medium">Migrate repositories</h2>
                <p className="text-xs text-muted-foreground leading-normal">
                  Bring your GitHub repositories to gitdot.
                </p>
                <p className="text-xs text-muted-foreground leading-normal mt-1">
                  All code and commit history will be preserved and private
                  repos will stay private.
                </p>
                <p className="text-xs text-muted-foreground leading-normal mt-1">
                  There are two types of migrations:
                </p>
                <ul className="text-xs text-muted-foreground leading-normal flex flex-col mt-1 gap-2 list-disc pl-4">
                  <li>
                    <span className="text-foreground">Read-only:</span> A
                    one-way sync. New commits made on GitHub are replicated to
                    gitdot and you <b>cannot</b> push to the gitdot repository.
                  </li>
                  <li>
                    <span className="text-foreground">Read-write:</span> A
                    one-time migration, giving you a fully functioning gitdot
                    repository to work out of.
                  </li>
                </ul>
                <p className="text-xs text-muted-foreground leading-normal mt-1">
                  Read-only repositories may be promoted to read-write at any
                  time.
                </p>
              </div>
              <div className="flex flex-col">
                {TYPE_OPTIONS.map((option) => {
                  const selected = migrationType === option.value;
                  return (
                    <button
                      key={option.value}
                      type="button"
                      onClick={() => setMigrationType(option.value)}
                      className="flex items-center gap-1.5 p-2 text-left text-xs border-t border-border/50 hover:bg-accent transition-colors duration-150 cursor-pointer"
                    >
                      <div
                        className={cn(
                          "shrink-0 w-3 h-3 rounded-xs border border-border transition-colors duration-150",
                          selected ? "bg-foreground" : "bg-background",
                        )}
                      />
                      <span>{option.label}</span>
                    </button>
                  );
                })}
              </div>
            </div>
          </div>
          <div className="flex items-center justify-between h-7">
            <span className="pl-2 text-xs truncate text-muted-foreground">
              {selectedRepos.size > 0
                ? `Migrate ${selectedRepos.size} ${selectedRepos.size === 1 ? "repository" : "repositories"} from GitHub`
                : "Migrate repositories from GitHub"}
            </span>
            <div className="flex items-center h-full">
              <button
                type="button"
                onClick={() => setOpen(false)}
                className="flex items-center px-2 h-full text-xs border-l border-border hover:bg-accent/50 transition-colors cursor-pointer"
              >
                Cancel
              </button>
              <button
                type="button"
                disabled={!isValid}
                className="flex items-center px-3 h-full text-xs bg-primary text-primary-foreground border-l border-primary enabled:hover:opacity-90 disabled:opacity-60 transition-opacity disabled:cursor-not-allowed cursor-pointer"
              >
                Migrate
              </button>
            </div>
          </div>
        </div>
      </DialogContent>
    </Dialog>
  );
}
