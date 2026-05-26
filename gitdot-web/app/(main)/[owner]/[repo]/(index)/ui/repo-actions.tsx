"use client";

import type { RepositoryResource } from "gitdot-api";
import { Copy, Download, Star } from "lucide-react";
import { useOptimistic, useTransition } from "react";
import { toast } from "@/(main)/provider/toaster";
import { useUserContext } from "@/(main)/provider/user";
import {
  starRepositoryAction,
  unstarRepositoryAction,
} from "@/actions/repository";
import { cn } from "@/util";

export function RepoActions({
  repository,
}: {
  repository: RepositoryResource;
}) {
  const { requireAuth } = useUserContext();
  const [, startTransition] = useTransition();
  const [optimistic, setOptimistic] = useOptimistic(
    { starred: repository.user_star, count: repository.stars },
    (state, next: boolean) => ({
      starred: next,
      count: state.count + (next ? 1 : 0) - (state.starred ? 1 : 0),
    }),
  );

  const handleStar = () => {
    if (requireAuth()) return;

    const next = !optimistic.starred;
    startTransition(async () => {
      setOptimistic(next);
      const result = next
        ? await starRepositoryAction(repository.owner, repository.name)
        : await unstarRepositoryAction(repository.owner, repository.name);
      if ("error" in result) {
        toast(result.error);
      }
    });
  };

  const handleClone = () => {
    const url = `${window.location.origin}/${repository.owner}/${repository.name}`;
    navigator.clipboard.writeText(url);
    toast(
      <div className="flex flex-col gap-1">
        <span>Copied to clipboard</span>
        <span className="font-mono bg-accent text-foreground px-1 rounded self-start whitespace-nowrap">
          git clone {url}
        </span>
      </div>,
      {
        icon: <Copy className="size-4" />,
        style: { "--width": "max-content" } as React.CSSProperties,
      },
    );
  };

  return (
    <div className="flex flex-col py-2 border-b">
      <span className="flex items-center gap-1.5 text-xs text-muted-foreground font-mono px-2 mb-1">
        Actions
      </span>
      <RepoActionButton
        icon={
          <Star
            className="size-3"
            fill={optimistic.starred ? "currentColor" : "none"}
          />
        }
        label={optimistic.starred ? "Starred" : "Star"}
        count={optimistic.count}
        active={optimistic.starred}
        onClick={handleStar}
      />
      <RepoActionButton
        icon={<Download className="size-3" />}
        label="Clone"
        onClick={handleClone}
      />
    </div>
  );
}

function RepoActionButton({
  icon,
  label,
  count,
  active,
  primary,
  onClick,
  className,
}: {
  icon: React.ReactNode;
  label: string;
  count?: number;
  active?: boolean;
  primary?: boolean;
  onClick: () => void;
  className?: string;
}) {
  return (
    <button
      type="button"
      onClick={onClick}
      className={cn(
        "w-full h-6 flex items-center justify-start gap-1.5 px-2 text-xs font-mono cursor-default transition-colors focus:outline-none",
        primary
          ? "bg-highlight text-highlight-foreground hover:bg-highlight/90"
          : active
            ? "bg-accent text-foreground"
            : "hover:bg-accent text-foreground",
        className,
      )}
    >
      {icon}
      <span>{label}</span>
      {count !== undefined && (
        <span className="ml-auto tabular-nums">{count}</span>
      )}
    </button>
  );
}
