"use client";

import { ChevronDown } from "lucide-react";
import { useState } from "react";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/ui/dropdown-menu";

const repos = ["gitdot", "async", "ollama"];

export function RepoSwitcher() {
  const [selectedRepo, setSelectedRepo] = useState(repos[0]);

  return (
    <DropdownMenu>
      <DropdownMenuTrigger asChild>
        <button
          type="button"
          className="flex items-center pl-1 gap-1 text-sm font-medium hover:opacity-80"
        >
          <span>{selectedRepo}</span>
          <ChevronDown className="h-3 w-3" />
        </button>
      </DropdownMenuTrigger>
      <DropdownMenuContent
        className="w-(--radix-dropdown-menu-trigger-width)"
        align="start"
      >
        {repos.map((repo) => (
          <DropdownMenuItem key={repo} onSelect={() => setSelectedRepo(repo)}>
            {repo}
          </DropdownMenuItem>
        ))}
      </DropdownMenuContent>
    </DropdownMenu>
  );
}
