"use client";

import {
  SidebarGroup,
  SidebarGroupContent,
  SidebarMenu,
  SidebarMenuItem,
} from "@/ui/sidebar";

const commits = [
  {
    hash: "73a88ff",
    message: "alt branch v1, separate header for toggles",
    author: "pybae",
    time: "2h ago",
  },
  {
    hash: "bc0158a",
    message: "mocking things out",
    author: "pybae",
    time: "3h ago",
  },
  {
    hash: "350877e",
    message: "prototyping the issues / pull requests page thing..",
    author: "pybae",
    time: "5h ago",
  },
  {
    hash: "9e058a7",
    message: "prototyping switcher icon one",
    author: "pybae",
    time: "1d ago",
  },
  { hash: "4694f8a", message: "a start...", author: "pybae", time: "1d ago" },
];

export function RepoCommits() {
  return (
    <SidebarGroup>
      <SidebarGroupContent>
        <SidebarMenu>
          {commits.map((commit) => (
            <SidebarMenuItem key={commit.hash}>
              <button
                type="button"
                className="px-2 py-1 text-sm text-sidebar-foreground/70 hover:text-sidebar-foreground hover:bg-sidebar-accent rounded-sm transition-colors w-full text-left flex flex-col"
              >
                <span className="truncate">{commit.message}</span>
                <span className="text-xs flex items-center justify-between text-sidebar-foreground/50">
                  <span className="font-mono">{commit.hash}</span>
                  <span className="flex items-center gap-1">
                    <span>{commit.author}</span>
                    <span>•</span>
                    <span>{commit.time}</span>
                  </span>
                </span>
              </button>
            </SidebarMenuItem>
          ))}
        </SidebarMenu>
      </SidebarGroupContent>
    </SidebarGroup>
  );
}
