"use client";

import type {
  RepositoryCommitFilterResource,
  RepositoryCommitResource,
  RepositoryPathsResource,
} from "gitdot-api";
import { ChevronDown, ChevronRight, Circle, X } from "lucide-react";
import { useRef, useState, useTransition } from "react";
import { toast } from "@/(main)/context/toaster";
import { useUserContext } from "@/(main)/context/user";
import { updateRepositoryCommitFilterAction } from "@/actions";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/ui/dropdown-menu";
import { cn } from "@/util";
import { ALL_COMMITS_FILTER, computePathOptions } from "../util";
import { SaveFilterDialog } from "./save-filter-dialog";

const TAG_OPTIONS = [
  "feat:",
  "fix:",
  "bug:",
  "refactor:",
  "chore:",
  "perf:",
  "test:",
  "docs:",
  "style:",
  "build:",
  "ci:",
  "revert:",
];

export function CommitsFilterDetail({
  owner,
  repo,
  commits,
  paths,
  filter,
  setActiveFilter,
  isModified,
}: {
  owner: string;
  repo: string;
  commits: RepositoryCommitResource[];
  paths: RepositoryPathsResource | null;
  filter: RepositoryCommitFilterResource;
  setActiveFilter: (filter: RepositoryCommitFilterResource) => void;
  isModified: boolean;
}) {
  const authors = filter.authors ?? [];
  const filterPaths = filter.paths ?? [];
  const tags = filter.tags ?? [];

  const authorOptions = Array.from(
    new Set(commits.map((c) => c.author.name ?? c.author.git_name)),
  ).sort();

  const pathOptions = computePathOptions(paths?.entries ?? [], commits);

  const toggleAuthor = (a: string) => {
    const next = authors.includes(a)
      ? authors.filter((x) => x !== a)
      : [...authors, a];
    setActiveFilter({ ...filter, authors: next });
  };
  const toggleTag = (t: string) => {
    const next = tags.includes(t) ? tags.filter((x) => x !== t) : [...tags, t];
    setActiveFilter({ ...filter, tags: next });
  };
  const addPath = (p: string) => {
    if (filterPaths.includes(p)) return;
    setActiveFilter({ ...filter, paths: [...filterPaths, p] });
  };
  const removePath = (p: string) => {
    setActiveFilter({
      ...filter,
      paths: filterPaths.filter((x) => x !== p),
    });
  };

  const isDefault = filter.id === ALL_COMMITS_FILTER.id;

  const { user, memberships } = useUserContext();
  const canSave =
    user?.name === owner ||
    (memberships ?? []).some((m) => m.org_name === owner);

  return (
    <div className="flex flex-col flex-1 min-h-0 overflow-y-auto">
      <NameCriteria
        value={filter.name}
        disabled={isDefault}
        onChange={(name) => setActiveFilter({ ...filter, name })}
      />
      <ChecklistCriteria
        label="Authors"
        options={authorOptions}
        selected={authors}
        onToggle={toggleAuthor}
        emptyLabel="All authors"
      />
      <ChecklistCriteria
        label="Tags"
        options={TAG_OPTIONS}
        selected={tags}
        onToggle={toggleTag}
        emptyLabel="Any message"
      />
      <PathsCriteria
        options={pathOptions}
        selected={filterPaths}
        onAdd={addPath}
        onRemove={removePath}
      />
      {canSave && (
        <SaveFilterButton
          owner={owner}
          repo={repo}
          filter={filter}
          enabled={isModified}
          isDefault={isDefault}
          setActiveFilter={setActiveFilter}
        />
      )}
    </div>
  );
}

function NameCriteria({
  value,
  disabled,
  onChange,
}: {
  value: string;
  disabled: boolean;
  onChange: (value: string) => void;
}) {
  return (
    <div className="flex flex-col gap-0.5 px-2 py-1.5 shrink-0 border-b border-border">
      <span className="text-[10px] text-muted-foreground uppercase tracking-wide font-mono select-none">
        Name
      </span>
      <input
        type="text"
        value={value}
        onChange={(e) => onChange(e.target.value)}
        disabled={disabled}
        spellCheck={false}
        autoComplete="off"
        className={cn(
          "text-xs font-mono bg-transparent outline-none w-full",
          disabled
            ? "text-muted-foreground/40 cursor-not-allowed"
            : "text-foreground",
        )}
      />
    </div>
  );
}

function ChecklistCriteria({
  label,
  options,
  selected,
  onToggle,
  emptyLabel,
}: {
  label: string;
  options: string[];
  selected: string[];
  onToggle: (value: string) => void;
  emptyLabel: string;
}) {
  const [open, setOpen] = useState(false);
  const count = selected.length;
  const summary = count > 0 ? selected.join(", ") : emptyLabel;
  const Chevron = open ? ChevronDown : ChevronRight;
  return (
    <DropdownMenu open={open} onOpenChange={setOpen}>
      <DropdownMenuTrigger className="w-full flex items-start justify-between gap-2 px-2 py-1.5 text-left shrink-0 border-b border-border hover:bg-accent/50 transition-colors focus:outline-none">
        <div className="flex flex-col min-w-0 gap-0.5">
          <span className="text-[10px] text-muted-foreground uppercase tracking-wide font-mono select-none">
            {label}
            {count > 0 ? ` (${count})` : ""}
          </span>
          <span
            className={cn(
              "text-xs font-mono truncate",
              count > 0 ? "text-foreground" : "text-muted-foreground/40",
            )}
          >
            {summary}
          </span>
        </div>
        <Chevron className="size-3 text-muted-foreground shrink-0 mt-1" />
      </DropdownMenuTrigger>
      <DropdownMenuContent side="bottom" align="start" className="w-56">
        {options.length === 0 ? (
          <div className="px-2 py-1.5 text-xs text-muted-foreground font-mono">
            No options
          </div>
        ) : (
          options.map((opt) => (
            <DropdownMenuItem
              key={opt}
              onSelect={(e) => e.preventDefault()}
              onClick={() => onToggle(opt)}
              className="text-xs font-mono"
            >
              <Circle
                className={cn(
                  "size-1.5 shrink-0",
                  selected.includes(opt)
                    ? "fill-current text-foreground"
                    : "fill-none text-muted-foreground",
                )}
              />
              {opt}
            </DropdownMenuItem>
          ))
        )}
      </DropdownMenuContent>
    </DropdownMenu>
  );
}

function PathsCriteria({
  options,
  selected,
  onAdd,
  onRemove,
}: {
  options: Array<{ path: string; count: number }>;
  selected: string[];
  onAdd: (path: string) => void;
  onRemove: (path: string) => void;
}) {
  const [query, setQuery] = useState("");
  const [focused, setFocused] = useState(false);
  const inputRef = useRef<HTMLInputElement>(null);

  const addPath = (p: string) => {
    onAdd(p);
    setQuery("");
    inputRef.current?.blur();
  };

  const suggestions = options.filter(
    (o) =>
      (query.length === 0 ||
        o.path.toLowerCase().includes(query.toLowerCase())) &&
      !selected.includes(o.path),
  );

  const showSuggestions = focused && suggestions.length > 0;

  return (
    <div className="relative flex flex-col gap-1 px-2 py-1.5 shrink-0 border-b border-border">
      <span className="text-[10px] text-muted-foreground uppercase tracking-wide font-mono select-none">
        Paths{selected.length > 0 ? ` (${selected.length})` : ""}
      </span>
      {selected.map((path) => (
        <div key={path} className="flex items-center justify-between gap-1">
          <span className="text-xs font-mono text-foreground truncate">
            {path}
          </span>
          <button
            type="button"
            onClick={() => onRemove(path)}
            className="text-muted-foreground hover:text-foreground shrink-0"
          >
            <X className="size-3" />
          </button>
        </div>
      ))}
      <input
        ref={inputRef}
        value={query}
        onChange={(e) => setQuery(e.target.value)}
        onFocus={() => setFocused(true)}
        onBlur={() => setFocused(false)}
        onKeyDown={(e) => {
          if (e.key === "Enter" && suggestions.length > 0) {
            addPath(suggestions[0].path);
          }
        }}
        placeholder="Search paths..."
        className="text-xs bg-transparent placeholder:text-muted-foreground/40 focus:outline-none w-full font-mono text-foreground"
      />
      {showSuggestions && (
        <div className="absolute left-0 right-0 top-full z-10 bg-popover border border-border shadow-md max-h-48 overflow-y-auto">
          {suggestions.map((s) => (
            <button
              key={s.path}
              type="button"
              onMouseDown={(e) => {
                e.preventDefault();
                addPath(s.path);
              }}
              className="flex items-center gap-1 px-2 h-6 w-full text-left font-mono text-xs text-muted-foreground hover:text-foreground hover:bg-accent/50 border-b border-border last:border-b-0"
            >
              <span className="truncate">{s.path}</span>
              <span className="text-muted-foreground/60 shrink-0 ml-auto">
                ({s.count})
              </span>
            </button>
          ))}
        </div>
      )}
    </div>
  );
}

function SaveFilterButton({
  owner,
  repo,
  filter,
  enabled,
  isDefault,
  setActiveFilter,
}: {
  owner: string;
  repo: string;
  filter: RepositoryCommitFilterResource;
  enabled: boolean;
  isDefault: boolean;
  setActiveFilter: (filter: RepositoryCommitFilterResource) => void;
}) {
  const [dialogOpen, setDialogOpen] = useState(false);
  const [isPending, startTransition] = useTransition();

  const interactable = enabled && !isPending;

  const saveInPlace = () => {
    if (isDefault || !interactable) return;
    startTransition(async () => {
      const result = await updateRepositoryCommitFilterAction(
        owner,
        repo,
        filter.id,
        {
          name: filter.name,
          authors: filter.authors,
          tags: filter.tags,
          paths: filter.paths,
        },
      );
      if ("filter" in result) {
        setActiveFilter(result.filter);
        toast.success("Filter saved");
      } else {
        toast.error(result.error);
      }
    });
  };

  return (
    <div className="flex justify-end px-2 py-2 shrink-0">
      {isDefault ? (
        <button
          type="button"
          onClick={() => setDialogOpen(true)}
          disabled={!interactable}
          className={cn(
            "px-2.5 h-6 text-xs font-mono bg-primary text-primary-foreground border border-border rounded-xs focus:outline-none transition-opacity",
            interactable
              ? "hover:opacity-80 cursor-pointer"
              : "opacity-50 cursor-not-allowed",
          )}
        >
          Save
        </button>
      ) : (
        <DropdownMenu>
          <DropdownMenuTrigger
            disabled={!interactable}
            className={cn(
              "flex items-center gap-0.75 px-2.5 h-6 text-xs font-mono bg-primary text-primary-foreground border border-border rounded-xs focus:outline-none transition-opacity",
              interactable
                ? "hover:opacity-80 cursor-pointer"
                : "opacity-50 cursor-not-allowed",
            )}
          >
            {isPending ? "Saving..." : "Save"}
            <ChevronDown className="size-3 mt-px" />
          </DropdownMenuTrigger>
          <DropdownMenuContent side="bottom" align="end" className="w-40">
            <DropdownMenuItem
              onClick={saveInPlace}
              className="text-xs font-mono"
            >
              Save
            </DropdownMenuItem>
            <DropdownMenuItem
              onClick={() => setDialogOpen(true)}
              className="text-xs font-mono"
            >
              Save as new
            </DropdownMenuItem>
          </DropdownMenuContent>
        </DropdownMenu>
      )}
      <SaveFilterDialog
        open={dialogOpen}
        setOpen={setDialogOpen}
        owner={owner}
        repo={repo}
        initialName={isDefault ? "" : filter.name}
        authors={filter.authors}
        tags={filter.tags}
        paths={filter.paths}
        onSaved={setActiveFilter}
      />
    </div>
  );
}
