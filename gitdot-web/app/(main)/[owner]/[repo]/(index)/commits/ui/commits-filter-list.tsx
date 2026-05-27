"use client";

import type { RepositoryCommitFilterResource } from "gitdot-api";
import { useState } from "react";
import { useUserContext } from "@/(main)/context/user";
import {
  ContextMenu,
  ContextMenuContent,
  ContextMenuItem,
  ContextMenuTrigger,
} from "@/ui/context-menu";
import { cn } from "@/util";
import { ALL_COMMITS_FILTER } from "../util";
import { DeleteFilterDialog } from "./delete-filter-dialog";

export function CommitsFilterList({
  owner,
  repo,
  filters,
  activeFilter,
  setActiveFilter,
  isModified,
}: {
  owner: string;
  repo: string;
  filters: RepositoryCommitFilterResource[];
  activeFilter: RepositoryCommitFilterResource;
  setActiveFilter: (filter: RepositoryCommitFilterResource) => void;
  isModified: boolean;
}) {
  const [filterToDelete, setFilterToDelete] =
    useState<RepositoryCommitFilterResource | null>(null);

  const { user, memberships } = useUserContext();
  const canSave =
    user?.name === owner ||
    (memberships ?? []).some((m) => m.org_name === owner);

  return (
    <div className="flex flex-col h-42 shrink-0 border-b border-border">
      <div className="flex items-center h-6 px-2 shrink-0 border-b border-border">
        <span className="text-xs text-muted-foreground font-mono">Filters</span>
      </div>
      <div className="flex flex-col flex-1 min-h-0 overflow-y-auto">
        {filters.map((filter) => {
          const isActive = activeFilter.id === filter.id;
          const isDefault = filter.id === ALL_COMMITS_FILTER.id;
          const button = (
            <button
              type="button"
              onClick={() => setActiveFilter(filter)}
              className={cn(
                "w-full flex flex-row items-center h-6 px-2 text-xs text-left transition-colors shrink-0 border-b border-border font-mono",
                isActive
                  ? "bg-accent text-foreground"
                  : "text-muted-foreground hover:bg-accent/50 hover:text-foreground",
              )}
            >
              {filter.name}
              {isActive && isModified ? " (*)" : ""}
            </button>
          );

          if (isDefault || !canSave) {
            return <div key={filter.id}>{button}</div>;
          }

          return (
            <ContextMenu key={filter.id}>
              <ContextMenuTrigger asChild>{button}</ContextMenuTrigger>
              <ContextMenuContent>
                <ContextMenuItem
                  variant="destructive"
                  onClick={() => setFilterToDelete(filter)}
                >
                  Delete
                </ContextMenuItem>
              </ContextMenuContent>
            </ContextMenu>
          );
        })}
      </div>
      <DeleteFilterDialog
        open={filterToDelete !== null}
        setOpen={(open) => {
          if (!open) setFilterToDelete(null);
        }}
        owner={owner}
        repo={repo}
        filter={filterToDelete}
        onDeleted={(deletedId) => {
          if (activeFilter.id === deletedId) {
            setActiveFilter(ALL_COMMITS_FILTER);
          }
        }}
      />
    </div>
  );
}
