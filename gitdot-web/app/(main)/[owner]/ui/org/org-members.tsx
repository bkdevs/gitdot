"use client";

import type { OrganizationMemberResource } from "gitdot-api";
import { ChevronDown } from "lucide-react";
import { useMemo, useState } from "react";
import { useTimezone } from "@/(main)/context/timezone";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/ui/dropdown-menu";
import Link from "@/ui/link";
import { formatDate } from "@/util/date";
import { UserImage } from "../user/user-image";

type MemberSort = "newest" | "oldest";

const MEMBER_SORT_LABELS: Record<MemberSort, string> = {
  newest: "Newest",
  oldest: "Oldest",
};

export function OrgMembers({
  members,
}: {
  members: OrganizationMemberResource[] | null;
}) {
  const tz = useTimezone();
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
        {sortedMembers.map((member) => (
          <div
            key={member.id}
            className="grid grid-cols-[auto_1fr_auto] gap-x-3"
          >
            <Link
              href={`/${member.user_name}`}
              className="peer row-span-2 self-start"
            >
              <UserImage
                userId={member.user_id}
                updatedAt={member.image_updated_at}
                px={32}
              />
            </Link>
            <Link
              href={`/${member.user_name}`}
              className="text-sm font-medium dark:font-normal underline decoration-transparent hover:decoration-current peer-hover:decoration-current transition-colors duration-200 self-start"
            >
              {member.user_name}
            </Link>
            <span className="text-xs font-mono text-muted-foreground self-start">
              joined {formatDate(new Date(member.created_at), tz)}
            </span>
            <p
              className={`col-start-2 col-end-4 ${
                member.role_description
                  ? "text-xs text-foreground"
                  : "text-xs text-muted-foreground italic"
              }`}
            >
              {member.role_description || "no description"}
            </p>
          </div>
        ))}
      </div>
    </div>
  );
}
