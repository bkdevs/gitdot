"use client";

import Link from "next/link";
import { useParams } from "next/navigation";
import {
  SidebarGroup,
  SidebarGroupContent,
  SidebarMenu,
  SidebarMenuItem,
} from "@/ui/sidebar";

const EXAMPLE_ISSUES = [
  { id: 1, title: "Fix authentication bug in login flow" },
  { id: 2, title: "Add dark mode support" },
  { id: 3, title: "Improve performance of data grid" },
  { id: 4, title: "Update dependencies to latest versions" },
  { id: 5, title: "Add unit tests for API endpoints" },
  { id: 6, title: "Refactor component structure" },
  { id: 7, title: "Fix mobile responsiveness issues" },
  { id: 8, title: "Add TypeScript strict mode" },
  { id: 9, title: "Implement user profile page" },
  { id: 10, title: "Fix memory leak in dashboard" },
  { id: 11, title: "Add documentation for API" },
  { id: 12, title: "Improve error handling" },
];

export function RepoIssues() {
  const params = useParams();
  const slug = params.slug as string;

  return (
    <SidebarGroup>
      <SidebarGroupContent>
        <SidebarMenu>
          {EXAMPLE_ISSUES.map((issue) => (
            <SidebarMenuItem key={issue.id}>
              <Link
                href={`/${slug}/issues/${issue.id}`}
                className="px-2 py-1 text-sm text-sidebar-foreground/70 hover:text-sidebar-foreground hover:bg-sidebar-accent rounded-sm block transition-colors truncate"
              >
                #{issue.id}: {issue.title}
              </Link>
            </SidebarMenuItem>
          ))}
        </SidebarMenu>
      </SidebarGroupContent>
    </SidebarGroup>
  );
}
