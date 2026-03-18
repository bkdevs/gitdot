"use client";

import {
    DropdownMenu,
    DropdownMenuCheckboxItem,
    DropdownMenuContent,
    DropdownMenuLabel,
    DropdownMenuTrigger,
} from "@/ui/dropdown-menu";
import { cn } from "@/util";
import { ChevronDown } from "lucide-react";
import { useState } from "react";

const MOCK_AUTHORS = ["Alice", "Bob", "Charlie"];
const MOCK_TAGS = ["Frontend", "Backend"];

export function CommitsHeader() {
  const [activeTags, setActiveTags] = useState<Set<string>>(new Set());

  function toggleTag(tag: string) {
    setActiveTags((prev) => {
      const next = new Set(prev);
      if (next.has(tag)) next.delete(tag);
      else next.add(tag);
      return next;
    });
  }

  return (
    <div className="flex flex-row w-full h-9 items-center border-b">
      <AuthorDropdown />
      {MOCK_TAGS.map((tag) => (
        <TagButton
          key={tag}
          label={tag}
          isActive={activeTags.has(tag)}
          onClick={() => toggleTag(tag)}
        />
      ))}
      <div className="ml-auto h-full flex flex-row">
        <DateRangeDropdown />
      </div>
    </div>
  );
}

function TagButton({
  label,
  isActive,
  onClick,
}: {
  label: string;
  isActive: boolean;
  onClick: () => void;
}) {
  return (
    <button
      type="button"
      className={cn(
        "flex flex-row items-center h-full border-border border-r px-2 text-xs hover:bg-sidebar",
        isActive ? "bg-sidebar text-foreground" : "text-muted-foreground",
      )}
      onClick={onClick}
    >
      {label}
    </button>
  );
}

function AuthorDropdown() {
  return (
    <DropdownMenu>
      <DropdownMenuTrigger asChild>
        <button
          type="button"
          className="flex flex-row items-center h-full border-border border-r px-2 text-xs text-muted-foreground hover:bg-sidebar"
        >
          Author: All
          <ChevronDown className="size-3 ml-1.5" />
        </button>
      </DropdownMenuTrigger>
      <DropdownMenuContent align="start">
        {MOCK_AUTHORS.map((author) => (
          <DropdownMenuCheckboxItem key={author} className="text-xs">
            {author}
          </DropdownMenuCheckboxItem>
        ))}
      </DropdownMenuContent>
    </DropdownMenu>
  );
}

function DateRangeDropdown() {
  return (
    <DropdownMenu>
      <DropdownMenuTrigger asChild>
        <button
          type="button"
          className="flex flex-row h-full items-center border-border border-l px-2 text-xs text-muted-foreground hover:bg-sidebar"
        >
          Jan 1, 2025 - Mar 18, 2026
          <ChevronDown className="size-3 ml-1.5" />
        </button>
      </DropdownMenuTrigger>
      <DropdownMenuContent align="end">
        <DropdownMenuLabel className="text-xs font-normal text-muted-foreground">
          Date range picker coming soon
        </DropdownMenuLabel>
      </DropdownMenuContent>
    </DropdownMenu>
  );
}
