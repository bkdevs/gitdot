"use client";

import Link from "next/link";
import { useParams } from "next/navigation";
import {
  SidebarGroup,
  SidebarGroupContent,
  SidebarMenu,
  SidebarMenuItem,
} from "@/ui/sidebar";

const EXAMPLE_PULLS = [
  { id: 1, title: "feat: Add user authentication system" },
  { id: 2, title: "fix: Resolve memory leak in data fetching" },
  { id: 3, title: "refactor: Modernize component architecture" },
  { id: 4, title: "chore: Update all dependencies" },
  { id: 5, title: "feat: Implement dark theme toggle" },
  { id: 6, title: "test: Add integration tests for API" },
  { id: 7, title: "docs: Update README with setup instructions" },
  { id: 8, title: "perf: Optimize image loading" },
  { id: 9, title: "feat: Add search functionality" },
  { id: 10, title: "fix: Correct TypeScript type errors" },
  { id: 11, title: "style: Improve mobile responsiveness" },
  { id: 12, title: "feat: Add user notifications" },
];

export function RepoPulls() {
  const params = useParams();
  const slug = params.slug as string;

  return (
    <SidebarGroup>
      <SidebarGroupContent>
        <SidebarMenu>
          {EXAMPLE_PULLS.map((pull) => (
            <SidebarMenuItem key={pull.id}>
              <Link
                href={`/${slug}/pulls/${pull.id}`}
                className="px-2 py-1 text-sm text-sidebar-foreground/70 hover:text-sidebar-foreground hover:bg-sidebar-accent rounded-sm block transition-colors truncate"
              >
                #{pull.id}: {pull.title}
              </Link>
            </SidebarMenuItem>
          ))}
        </SidebarMenu>
      </SidebarGroupContent>
    </SidebarGroup>
  );
}
