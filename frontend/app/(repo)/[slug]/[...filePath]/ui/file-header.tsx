"use client";

import { Copy, GitBranch, History, Menu } from "lucide-react";
import Link from "next/link";
import type { RepositoryCommit, RepositoryFile } from "@/lib/dto";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/ui/dropdown-menu";
import { timeAgo } from "@/util";

export function FileHeader({
  repo,
  file,
  commit,
}: {
  repo: string;
  file: RepositoryFile;
  commit: RepositoryCommit;
}) {
  var path = "";
  const pathSegments = file.path.split("/");
  const pathLinks: React.ReactNode[] = [];
  pathSegments.forEach((segment, index) => {
    path += `/${segment}`;
    pathLinks.push(
      <Link className="hover:underline" href={`/${repo}${path}`} key={segment}>
        {segment}
      </Link>,
    );
    if (index !== pathSegments.length - 1) {
      pathLinks.push(<span key={`${segment}-separator`}>/</span>);
    }
  });

  const handleCopyFile = () => {
    // TODO: Implement copy file functionality
    console.log("Copy file");
  };

  const handleOpenBlame = () => {
    // TODO: Implement open blame functionality
    console.log("Open blame");
  };

  const handleShowHistory = () => {
    // TODO: Implement show history functionality
    console.log("Show history");
  };

  return (
    <div className="flex flex-row w-full h-9 items-center border-b">
      <DropdownMenu>
        <DropdownMenuTrigger asChild>
          <button
            type="button"
            className="flex w-9 border-r h-9 items-center justify-center hover:bg-accent"
          >
            <Menu className="size-3" />
          </button>
        </DropdownMenuTrigger>
        <DropdownMenuContent align="start">
          <DropdownMenuItem onClick={handleCopyFile}>
            <Copy />
            Copy file
          </DropdownMenuItem>
          <DropdownMenuItem onClick={handleOpenBlame}>
            <GitBranch />
            Open blame
          </DropdownMenuItem>
          <DropdownMenuItem onClick={handleShowHistory}>
            <History />
            Show history
          </DropdownMenuItem>
        </DropdownMenuContent>
      </DropdownMenu>
      <div className="ml-1 text-sm font-mono">{pathLinks}</div>
      <div className="ml-auto mr-3 text-sm font-mono">
        {commit.author} • {commit.message}
        <span className="text-muted-foreground">
          {timeAgo(new Date(commit.date))}
        </span>
      </div>
    </div>
  );
}
