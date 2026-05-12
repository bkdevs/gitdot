"use client";

import { ChevronDown, GitCommit, Star } from "lucide-react";
import { useState } from "react";
import { AvatarBeam } from "@/ui/avatar-beam";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/ui/dropdown-menu";
import Link from "@/ui/link";
import { timeAgo } from "@/util";

type ActivityFilter = "all" | "stars" | "commits";

const FILTER_LABELS: Record<ActivityFilter, string> = {
  all: "All",
  stars: "Stars",
  commits: "Commits",
};

type StarActivity = {
  kind: "star";
  id: string;
  userName: string;
  date: Date;
};

type CommitActivity = {
  kind: "commit";
  id: string;
  userName: string;
  message: string;
  sha: string;
  date: Date;
};

type Activity = StarActivity | CommitActivity;

const MIN = 60 * 1000;
const HR = 60 * MIN;
const DAY = 24 * HR;

const PLACEHOLDER_ACTIVITY: Activity[] = [
  {
    kind: "star",
    id: "1",
    userName: "alice-chen",
    date: new Date(Date.now() - 12 * MIN),
  },
  {
    kind: "commit",
    id: "2",
    userName: "bob-martin",
    message: "fix: handle empty repo edge case in path traversal",
    sha: "a1b2c3d",
    date: new Date(Date.now() - 47 * MIN),
  },
  {
    kind: "star",
    id: "3",
    userName: "marcus-aurelius",
    date: new Date(Date.now() - 2 * HR),
  },
  {
    kind: "commit",
    id: "4",
    userName: "carol-vance",
    message: "feat: add user activity feed with filters",
    sha: "e4f5g6h",
    date: new Date(Date.now() - 3 * HR),
  },
  {
    kind: "commit",
    id: "5",
    userName: "diego-ramirez",
    message: "refactor: extract diff parser into separate module",
    sha: "i7j8k9l",
    date: new Date(Date.now() - 5 * HR),
  },
  {
    kind: "star",
    id: "6",
    userName: "yuki-tanaka",
    date: new Date(Date.now() - 8 * HR),
  },
  {
    kind: "star",
    id: "7",
    userName: "priya-patel",
    date: new Date(Date.now() - 14 * HR),
  },
  {
    kind: "commit",
    id: "8",
    userName: "bob-martin",
    message: "chore: bump dependencies to latest",
    sha: "m0n1o2p",
    date: new Date(Date.now() - 19 * HR),
  },
  {
    kind: "commit",
    id: "9",
    userName: "evan-park",
    message: "docs: update README with new install steps",
    sha: "q3r4s5t",
    date: new Date(Date.now() - DAY - 2 * HR),
  },
  {
    kind: "star",
    id: "10",
    userName: "sofia-rossi",
    date: new Date(Date.now() - DAY - 6 * HR),
  },
  {
    kind: "commit",
    id: "11",
    userName: "carol-vance",
    message: "fix: race condition in commit hover state",
    sha: "u6v7w8x",
    date: new Date(Date.now() - 2 * DAY),
  },
  {
    kind: "commit",
    id: "12",
    userName: "linus-kovac",
    message: "perf: cache rendered markdown blobs",
    sha: "y9z0a1b",
    date: new Date(Date.now() - 2 * DAY - 4 * HR),
  },
  {
    kind: "star",
    id: "13",
    userName: "amara-okafor",
    date: new Date(Date.now() - 3 * DAY),
  },
  {
    kind: "commit",
    id: "14",
    userName: "diego-ramirez",
    message: "test: add coverage for blob diff edge cases",
    sha: "c2d3e4f",
    date: new Date(Date.now() - 3 * DAY - 8 * HR),
  },
  {
    kind: "star",
    id: "15",
    userName: "noah-fischer",
    date: new Date(Date.now() - 4 * DAY),
  },
  {
    kind: "commit",
    id: "16",
    userName: "evan-park",
    message: "style: tighten up button padding in repo panel",
    sha: "g5h6i7j",
    date: new Date(Date.now() - 5 * DAY),
  },
  {
    kind: "star",
    id: "17",
    userName: "hana-suzuki",
    date: new Date(Date.now() - 6 * DAY),
  },
  {
    kind: "star",
    id: "18",
    userName: "leo-bauer",
    date: new Date(Date.now() - 6 * DAY - 5 * HR),
  },
  {
    kind: "commit",
    id: "19",
    userName: "bob-martin",
    message: "feat: implement search across owners and repos",
    sha: "k8l9m0n",
    date: new Date(Date.now() - 7 * DAY),
  },
  {
    kind: "star",
    id: "20",
    userName: "isabela-cruz",
    date: new Date(Date.now() - 9 * DAY),
  },
  {
    kind: "commit",
    id: "21",
    userName: "carol-vance",
    message: "fix: prevent double submission on action buttons",
    sha: "o1p2q3r",
    date: new Date(Date.now() - 11 * DAY),
  },
  {
    kind: "star",
    id: "22",
    userName: "theo-wagner",
    date: new Date(Date.now() - 13 * DAY),
  },
  {
    kind: "commit",
    id: "23",
    userName: "linus-kovac",
    message: "refactor: collapse provider context into single hook",
    sha: "s4t5u6v",
    date: new Date(Date.now() - 14 * DAY),
  },
  {
    kind: "star",
    id: "24",
    userName: "fatima-nasser",
    date: new Date(Date.now() - 18 * DAY),
  },
  {
    kind: "commit",
    id: "25",
    userName: "diego-ramirez",
    message: "chore: drop unused legacy diff renderer",
    sha: "w7x8y9z",
    date: new Date(Date.now() - 22 * DAY),
  },
];

export function RepoActivity() {
  const [filter, setFilter] = useState<ActivityFilter>("all");

  const items = PLACEHOLDER_ACTIVITY.filter((a) => {
    if (filter === "all") return true;
    if (filter === "stars") return a.kind === "star";
    return a.kind === "commit";
  });

  return (
    <div className="flex-1 min-h-0 flex flex-col px-3 pt-3">
      <div className="flex items-baseline justify-between mb-2">
        <span className="text-xs text-muted-foreground font-mono">
          <span className="text-foreground/40 select-none"># </span>
          Activity
        </span>
        <DropdownMenu>
          <DropdownMenuTrigger className="flex items-center gap-0.5 text-xs text-muted-foreground/60 font-mono cursor-pointer transition-colors hover:text-foreground">
            {FILTER_LABELS[filter]}
            <ChevronDown className="size-3" />
          </DropdownMenuTrigger>
          <DropdownMenuContent align="end" className="min-w-20">
            {(Object.keys(FILTER_LABELS) as ActivityFilter[]).map((key) => (
              <DropdownMenuItem
                key={key}
                className="text-xs font-mono"
                onClick={() => setFilter(key)}
              >
                {FILTER_LABELS[key]}
              </DropdownMenuItem>
            ))}
          </DropdownMenuContent>
        </DropdownMenu>
      </div>
      <div className="flex flex-col gap-3 overflow-y-auto scrollbar-none">
        {items.map((item) => (
          <ActivityRow key={item.id} item={item} />
        ))}
        {items.length === 0 && (
          <span className="font-mono text-xs text-muted-foreground">
            no activity
          </span>
        )}
      </div>
    </div>
  );
}

function ActivityRow({ item }: { item: Activity }) {
  return (
    <div className="flex items-start gap-2 text-xs">
      <Link href={`/${item.userName}`} className="shrink-0 mt-0.5">
        <AvatarBeam name={item.userName} size={24} className="rounded-full" />
      </Link>
      <div className="flex flex-col min-w-0 flex-1">
        <div className="truncate">
          <Link
            href={`/${item.userName}`}
            className="font-medium hover:underline"
          >
            {item.userName}
          </Link>
          <span className="text-muted-foreground">
            {item.kind === "star" ? " starred" : " committed"}
          </span>
        </div>
        {item.kind === "commit" && (
          <span className="text-foreground truncate">{item.message}</span>
        )}
        <div className="flex items-center gap-1 text-[10px] font-mono text-muted-foreground">
          {item.kind === "star" ? (
            <Star className="size-3 shrink-0" />
          ) : (
            <GitCommit className="size-3 shrink-0" />
          )}
          <span>{timeAgo(item.date)}</span>
        </div>
      </div>
    </div>
  );
}
