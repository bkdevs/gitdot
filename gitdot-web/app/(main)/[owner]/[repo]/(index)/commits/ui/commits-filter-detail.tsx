"use client";

import type {
  RepositoryCommitFilterResource,
  RepositoryCommitResource,
  RepositoryPathsResource,
} from "gitdot-api";
import { ChevronRight, Circle, X } from "lucide-react";
import { useRef, useState } from "react";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/ui/dropdown-menu";
import { cn } from "@/util";

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
  commits,
  paths,
  filter,
  setActiveFilter,
  isModified,
}: {
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
    new Set(commits.map((c) => c.author.name)),
  ).sort();

  const pathOptions =
    paths?.entries.map((e) =>
      e.path_type === "tree" ? `${e.path}/` : e.path,
    ) ?? [];

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

  return (
    <div className="flex flex-col flex-1 min-h-0 overflow-y-auto">
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
      <SaveFilterButton enabled={isModified} />
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
  const count = selected.length;
  const summary = count > 0 ? selected.join(", ") : emptyLabel;
  return (
    <DropdownMenu>
      <DropdownMenuTrigger className="w-full flex items-start justify-between gap-2 px-2 py-1.5 text-left shrink-0 border-b border-border hover:bg-accent/50 transition-colors focus:outline-none">
        <div className="flex flex-col min-w-0 gap-0.5">
          <span className="text-[10px] text-muted-foreground uppercase tracking-wide font-mono">
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
        <ChevronRight className="size-3 text-muted-foreground shrink-0 mt-1" />
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
  options: string[];
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
      (query.length === 0 || o.toLowerCase().includes(query.toLowerCase())) &&
      !selected.includes(o),
  );

  const showSuggestions = focused && suggestions.length > 0;

  return (
    <div className="relative flex flex-col gap-1 px-2 py-1.5 shrink-0 border-b border-border">
      <span className="text-[10px] text-muted-foreground uppercase tracking-wide font-mono">
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
            addPath(suggestions[0]);
          }
        }}
        placeholder="Search paths..."
        className="text-xs bg-transparent placeholder:text-muted-foreground/40 focus:outline-none w-full font-mono text-foreground"
      />
      {showSuggestions && (
        <div className="absolute left-0 right-0 top-full z-10 bg-popover border border-border shadow-md max-h-48 overflow-y-auto">
          {suggestions.map((s) => (
            <button
              key={s}
              type="button"
              onMouseDown={(e) => {
                e.preventDefault();
                addPath(s);
              }}
              className="flex items-center px-2 h-6 w-full text-left font-mono text-xs text-muted-foreground hover:text-foreground hover:bg-accent/50 border-b border-border last:border-b-0"
            >
              {s}
            </button>
          ))}
        </div>
      )}
    </div>
  );
}

function SaveFilterButton({ enabled }: { enabled: boolean }) {
  return (
    <div className="flex justify-end px-2 py-2 shrink-0">
      <button
        type="button"
        disabled={!enabled}
        className={cn(
          "px-2.5 h-6 text-xs font-mono bg-primary text-primary-foreground border border-border rounded-xs focus:outline-none",
          enabled ? "hover:bg-primary/90" : "opacity-50 cursor-not-allowed",
        )}
      >
        Save filter
      </button>
    </div>
  );
}
