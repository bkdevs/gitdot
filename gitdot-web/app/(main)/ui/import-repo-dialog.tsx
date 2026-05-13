"use client";

import { VisuallyHidden } from "@radix-ui/react-visually-hidden";
import { ChevronDown } from "lucide-react";
import Image from "next/image";
import { useEffect, useState } from "react";
import { OrgImage } from "@/(main)/[owner]/ui/org/org-image";
import { UserImage } from "@/(main)/[owner]/ui/user/user-image";
import { useUserContext } from "@/(main)/context/user";
import { Dialog, DialogContent, DialogTitle } from "@/ui/dialog";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/ui/dropdown-menu";
import { cn } from "@/util";

type ImportType = "read-only" | "read-write";
type RepoSort = "recent" | "name";

const REPO_SORT_LABELS: Record<RepoSort, string> = {
  recent: "Recent",
  name: "Name",
};

const PLACEHOLDER_REPOS: { name: string; description: string }[] = Array.from(
  { length: 100 },
  (_, i) => ({
    name: `octocat/repo-${i + 1}`,
    description: `Placeholder repository number ${i + 1}`,
  }),
);

const TYPE_OPTIONS: {
  value: ImportType;
  label: string;
  description: string;
}[] = [
  {
    value: "read-only",
    label: "Read-only",
    description: "Mirror from GitHub. No pushes from gitdot.",
  },
  {
    value: "read-write",
    label: "Read-write",
    description: "Full migration. A regular gitdot repository, push freely.",
  },
];

export function ImportRepoDialog() {
  const { user, memberships } = useUserContext();

  const [open, setOpen] = useState(false);
  const [githubAccount, setGithubAccount] = useState("");
  const [destination, setDestination] = useState("");
  const [selectedRepos, setSelectedRepos] = useState<Set<string>>(new Set());
  const [importType, setImportType] = useState<ImportType>("read-only");
  const [sortBy, setSortBy] = useState<RepoSort>("recent");

  const selectedMembership = memberships?.find(
    (m) => m.org_name === destination,
  );

  useEffect(() => {
    if (!open) {
      setSelectedRepos(new Set());
      setImportType("read-only");
    }
  }, [open]);

  useEffect(() => {
    const handle = (e: Event) => {
      if (!user) return;
      const detail = (e as CustomEvent<{ owner?: string }>).detail;
      setGithubAccount(`github.com/${user.name}`);
      setDestination(detail?.owner ?? user.name);
      setOpen(true);
    };
    window.addEventListener("openImportRepo", handle);
    return () => window.removeEventListener("openImportRepo", handle);
  }, [user]);

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
        className="max-w-xl min-w-xl border-black rounded-xs shadow-2xl top-[45%] p-0 overflow-hidden"
        animations={true}
        showOverlay={true}
      >
        <VisuallyHidden>
          <DialogTitle>Import repositories</DialogTitle>
        </VisuallyHidden>
        <div className="relative">
          <div className="flex border-b border-border">
            <div className="flex flex-col w-2/3 p-2 border-r border-border">
              <h2 className="text-sm font-medium">Import repositories</h2>
              <p className="text-xs text-muted-foreground leading-normal">
                Bring your existing GitHub repositories to gitdot.
              </p>
              <p className="text-xs text-muted-foreground leading-normal mt-2">
                We'll import all code and commit history, and preserve each
                repository's visibility so private repos stay private. Read-only
                repos can be promoted to read-write when you're ready to
                migrate.
              </p>
            </div>
            <div className="flex flex-col w-1/3">
              {TYPE_OPTIONS.map((option, idx) => {
                const selected = importType === option.value;
                return (
                  <button
                    key={option.value}
                    type="button"
                    onClick={() => setImportType(option.value)}
                    className={cn(
                      "flex items-start gap-1.5 p-2 text-left hover:bg-accent transition-colors duration-150 cursor-pointer",
                      idx > 0 && "border-t border-border/50",
                    )}
                  >
                    <div
                      className={cn(
                        "mt-[3px] shrink-0 w-3 h-3 rounded-xs border border-border transition-colors duration-150",
                        selected ? "bg-foreground" : "bg-background",
                      )}
                    />
                    <div className="flex flex-col">
                      <span className="text-xs">{option.label}</span>
                      <span className="text-xs text-muted-foreground leading-normal">
                        {option.description}
                      </span>
                    </div>
                  </button>
                );
              })}
            </div>
          </div>
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
                    {githubAccount || "select"}
                    <ChevronDown className="size-3" />
                  </DropdownMenuTrigger>
                  <DropdownMenuContent align="start" className="min-w-32">
                    {user && (
                      <DropdownMenuItem
                        className="text-xs"
                        onClick={() =>
                          setGithubAccount(`github.com/${user.name}`)
                        }
                      >
                        <Image
                          src="/github-logo.svg"
                          alt=""
                          width={13}
                          height={13}
                        />
                        github.com/{user.name}
                      </DropdownMenuItem>
                    )}
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
              <span className="text-muted-foreground w-8 shrink-0">To:</span>
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
                  {destination ? `gitdot.io/${destination}` : "select"}
                  <ChevronDown className="size-3" />
                </DropdownMenuTrigger>
                <DropdownMenuContent align="start" className="min-w-32">
                  {user && (
                    <DropdownMenuItem
                      className="text-xs"
                      onClick={() => setDestination(user.name)}
                    >
                      <UserImage userId={user.id} px={14} />
                      gitdot.io/{user.name}
                    </DropdownMenuItem>
                  )}
                  {memberships?.map((m) => (
                    <DropdownMenuItem
                      key={m.organization_id}
                      className="text-xs"
                      onClick={() => setDestination(m.org_name)}
                    >
                      <OrgImage orgId={m.organization_id} px={14} />
                      gitdot.io/{m.org_name}
                    </DropdownMenuItem>
                  ))}
                </DropdownMenuContent>
              </DropdownMenu>
            </div>
          </div>
          <div className="flex items-center justify-between px-2 pt-1.5 pb-0.5 text-xs">
            <span className="text-muted-foreground">Repositories:</span>
            <DropdownMenu>
              <DropdownMenuTrigger className="flex items-center gap-0.5 text-muted-foreground/60 cursor-pointer transition-colors hover:text-foreground">
                {REPO_SORT_LABELS[sortBy]}
                <ChevronDown className="size-3" />
              </DropdownMenuTrigger>
              <DropdownMenuContent align="end" className="min-w-20">
                {(Object.keys(REPO_SORT_LABELS) as RepoSort[]).map((key) => (
                  <DropdownMenuItem
                    key={key}
                    className="text-xs"
                    onClick={() => setSortBy(key)}
                  >
                    {REPO_SORT_LABELS[key]}
                  </DropdownMenuItem>
                ))}
              </DropdownMenuContent>
            </DropdownMenu>
          </div>
          <div className="flex flex-col h-80 overflow-y-auto scrollbar-thin border-b border-border">
            {PLACEHOLDER_REPOS.map((repo) => {
              const checked = selectedRepos.has(repo.name);
              const shortName = repo.name.split("/").pop() ?? repo.name;
              return (
                <button
                  key={repo.name}
                  type="button"
                  onClick={() => toggleRepo(repo.name)}
                  className={cn(
                    "flex flex-col px-2 py-1.5 text-xs text-left hover:bg-accent transition-colors duration-150 cursor-pointer",
                    checked && "bg-accent",
                  )}
                >
                  <div className="flex items-center gap-1.5 min-w-0">
                    <span className="truncate">{repo.name}</span>
                    {checked && destination && (
                      <span className="text-muted-foreground truncate">
                        → {destination}/{shortName}
                      </span>
                    )}
                  </div>
                  <span className="truncate text-muted-foreground">
                    {repo.description}
                  </span>
                </button>
              );
            })}
          </div>
          <div className="flex items-center justify-between h-7">
            <span className="pl-2 text-xs truncate text-muted-foreground">
              {selectedRepos.size > 0
                ? `Import ${selectedRepos.size} ${selectedRepos.size === 1 ? "repository" : "repositories"} from GitHub`
                : "Import repositories from GitHub"}
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
                Import
              </button>
            </div>
          </div>
        </div>
      </DialogContent>
    </Dialog>
  );
}
