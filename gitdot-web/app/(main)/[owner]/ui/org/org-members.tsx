"use client";

import type { OrganizationMemberResource } from "gitdot-api";
import { ChevronDown } from "lucide-react";
import { useMemo, useState } from "react";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/ui/dropdown-menu";
import { formatDate } from "@/util/date";
import { UserImage } from "../user/user-image";

type MemberSort = "newest" | "oldest";

const MEMBER_SORT_LABELS: Record<MemberSort, string> = {
  newest: "Newest",
  oldest: "Oldest",
};

const MOCK_ROLE_DESCRIPTIONS = [
  "Leads infrastructure work and keeps the build pipeline healthy across all repositories.",
  "Focuses on frontend architecture and the design system. Reviews most UI-heavy pull requests.",
  "Owns the data platform and analytics tooling. Author of the migration framework.",
  "Maintains the public API and SDK packages. Writes most of the integration docs.",
];

function mockRoleDescription(index: number): string {
  return MOCK_ROLE_DESCRIPTIONS[index % MOCK_ROLE_DESCRIPTIONS.length];
}

export function OrgMembers({
  members,
}: {
  members: OrganizationMemberResource[] | null;
}) {
  const [sortBy, setSortBy] = useState<MemberSort>("oldest");

  const sortedMembers = useMemo(() => {
    if (!members) return [];
    return [...members].sort((a, b) => {
      const aTime = new Date(a.created_at).getTime();
      const bTime = new Date(b.created_at).getTime();
      return sortBy === "newest" ? bTime - aTime : aTime - bTime;
    });
  }, [members, sortBy]);

  if (!members?.length) return null;

  return (
    <div className="px-3">
      <div className="flex items-baseline justify-between mb-2">
        <span className="text-xs text-muted-foreground font-mono">
          <span className="text-foreground/40 select-none"># </span>
          Members
        </span>
        <DropdownMenu>
          <DropdownMenuTrigger className="flex items-center gap-0.5 text-xs text-muted-foreground/60 font-mono cursor-pointer transition-colors hover:text-foreground">
            {MEMBER_SORT_LABELS[sortBy]}
            <ChevronDown className="size-3" />
          </DropdownMenuTrigger>
          <DropdownMenuContent align="end" className="min-w-20">
            {(Object.keys(MEMBER_SORT_LABELS) as MemberSort[]).map((key) => (
              <DropdownMenuItem
                key={key}
                className="text-xs font-mono"
                onClick={() => setSortBy(key)}
              >
                {MEMBER_SORT_LABELS[key]}
              </DropdownMenuItem>
            ))}
          </DropdownMenuContent>
        </DropdownMenu>
      </div>
      <div className="flex flex-col gap-4">
        {sortedMembers.map((member, i) => (
          <div key={member.id} className="flex items-start gap-3">
            <UserImage userId={member.user_id} px={32} className="mt-0.5" />
            <div className="flex flex-col">
              <span className="text-sm font-medium">{member.user_name}</span>
              <p className="text-xs text-foreground -mt-0.5">
                {mockRoleDescription(i)}
              </p>
              <span className="text-xs font-mono text-muted-foreground mt-0.5">
                Joined {formatDate(new Date(member.created_at))}
              </span>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}
