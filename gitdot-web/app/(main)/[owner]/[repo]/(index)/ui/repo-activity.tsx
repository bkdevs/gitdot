"use client";

import type {
  GetRepositoryActivityResponse,
  RepositoryActivityEventResource,
  UserResource,
} from "gitdot-api";
import { Star } from "lucide-react";
import { Suspense, use, useMemo } from "react";
import Link from "@/ui/link";
import { timeAgo } from "@/util";

export function RepoActivity({
  activityPromise,
  starred,
  currentUser,
}: {
  activityPromise: Promise<GetRepositoryActivityResponse | null>;
  starred: boolean;
  currentUser: UserResource | null;
}) {
  return (
    <div className="flex-1 min-h-0 flex flex-col p-2">
      <span className="flex items-center gap-1.5 text-xs text-muted-foreground font-mono mb-2">
        Activity
      </span>
      <Suspense
        fallback={
          <span className="text-xs text-muted-foreground font-mono">
            loading...
          </span>
        }
      >
        <ActivityList
          promise={activityPromise}
          starred={starred}
          currentUser={currentUser}
        />
      </Suspense>
    </div>
  );
}

function ActivityList({
  promise,
  starred,
  currentUser,
}: {
  promise: Promise<GetRepositoryActivityResponse | null>;
  starred: boolean;
  currentUser: UserResource | null;
}) {
  const serverEvents = use(promise) ?? [];
  const events = useMemo(() => {
    if (!currentUser) return serverEvents;
    const userStarEvent = serverEvents.find(
      (e) => e.type === "starred" && e.user.id === currentUser.id,
    );
    const others = serverEvents.filter((e) => e !== userStarEvent);
    if (!starred) return others;
    const userEvent = {
      type: "starred" as const,
      user: currentUser,
      at: userStarEvent?.at ?? new Date().toISOString(),
    };
    return [...others, userEvent].sort(
      (a, b) => new Date(b.at).getTime() - new Date(a.at).getTime(),
    );
  }, [serverEvents, starred, currentUser]);

  if (events.length === 0) {
    return (
      <span className="text-xs text-muted-foreground font-mono">
        no activity yet
      </span>
    );
  }

  return (
    <div className="flex flex-col gap-2 overflow-y-auto scrollbar-none">
      {events.map((event, i) => (
        <ActivityRow key={activityKey(event, i)} event={event} />
      ))}
    </div>
  );
}

function activityKey(event: RepositoryActivityEventResource, i: number) {
  if (event.type === "starred") return `starred:${event.user.id}:${event.at}`;
  return `${i}`;
}

function ActivityRow({ event }: { event: RepositoryActivityEventResource }) {
  if (event.type === "starred") {
    return (
      <div className="flex items-center gap-1.5 min-w-0 text-xs">
        <div className="flex items-center gap-1 truncate min-w-0 flex-1">
          <Link
            href={`/${event.user.name}`}
            className="font-medium hover:underline truncate"
          >
            {event.user.name}
          </Link>
          <span className="text-muted-foreground">starred</span>
          <Star
            className="size-2.5 shrink-0 text-muted-foreground"
            fill="currentColor"
          />
        </div>
        <span className="shrink-0 font-mono text-muted-foreground">
          {timeAgo(new Date(event.at))}
        </span>
      </div>
    );
  }
  return null;
}
