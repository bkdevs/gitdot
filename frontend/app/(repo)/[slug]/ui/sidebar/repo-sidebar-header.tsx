"use client";

import { ChevronDown, GitBranch } from "lucide-react";
import { useState } from "react";

export function RepoSidebarHeader({ repo }: { repo: string }) {
  const [isOpen, setIsOpen] = useState(false);
  const [currentBranch, setCurrentBranch] = useState("main");

  const branches = ["main", "develop", "feature/new-ui", "hotfix/bug-123"];

  return (
    <div className="flex flex-row w-full h-9 items-center border-b justify-between">
      <span className="ml-2 text-sm font-medium">{repo}</span>

      <div className="relative h-full">
        <button
          type="button"
          onClick={() => setIsOpen(!isOpen)}
          className="flex items-center gap-1 px-2 h-full text-xs border-l hover:bg-gray-50 w-24"
        >
          <GitBranch className="size-3" />
          <span className="flex-1 truncate text-left">{currentBranch}</span>
          <ChevronDown className="size-3" />
        </button>

        {isOpen && (
          <div className="absolute right-0 mt-1 w-48 bg-white border rounded shadow-lg z-10">
            <div className="py-1">
              {branches.map((branch) => (
                <button
                  type="submit"
                  key={branch}
                  onClick={() => {
                    setCurrentBranch(branch);
                    setIsOpen(false);
                  }}
                  className={`w-full text-left px-3 py-1.5 text-xs hover:bg-gray-100 ${
                    branch === currentBranch ? "bg-gray-50 font-medium" : ""
                  }`}
                >
                  {branch}
                </button>
              ))}
            </div>
          </div>
        )}
      </div>
    </div>
  );
}
