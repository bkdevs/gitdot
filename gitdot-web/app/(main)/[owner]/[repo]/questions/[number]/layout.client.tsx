"use client";

import { Undo2 } from "lucide-react";
import { useParams } from "next/navigation";
import { Suspense, use } from "react";
import {
  type ResourcePromisesType,
  type ResourceRequestsType,
  useResolvePromises,
} from "@/(main)/[owner]/[repo]/resources";
import Link from "@/ui/link";
import { OverlayScroll } from "@/ui/scroll";
import { Sidebar, SidebarContent } from "@/ui/sidebar";
import { timeAgo } from "@/util";
import type { Resources } from "./layout";

type ResourceRequests = ResourceRequestsType<Resources>;
type ResourcePromises = ResourcePromisesType<Resources>;

export function LayoutClient({
  owner,
  repo,
  requests,
  promises,
  children,
}: {
  owner: string;
  repo: string;
  requests: ResourceRequests;
  promises: ResourcePromises;
  children: React.ReactNode;
}) {
  const resolvedPromises = useResolvePromises(owner, repo, requests, promises);
  return (
    <>
      <Sidebar>
        <SidebarContent className="overflow-auto">
          <div className="flex flex-col w-full">
            <QuestionSidebarHeader owner={owner} repo={repo} />
            <Suspense>
              <QuestionSidebarContent
                owner={owner}
                repo={repo}
                promises={resolvedPromises}
              />
            </Suspense>
          </div>
        </SidebarContent>
      </Sidebar>
      <OverlayScroll>{children}</OverlayScroll>
    </>
  );
}

function QuestionSidebarHeader({
  owner,
  repo,
}: {
  owner: string;
  repo: string;
}) {
  return (
    <Link
      href={`/${owner}/${repo}/questions`}
      className="sticky top-0 bg-background flex items-center justify-between border-b px-2 h-9 z-10 hover:bg-accent/50 cursor-default"
    >
      <h3 className="text-xs font-semibold text-muted-foreground uppercase tracking-wide">
        Questions
      </h3>
      <Undo2 size={14} className="text-muted-foreground -translate-y-px" />
    </Link>
  );
}

function QuestionSidebarContent({
  owner,
  repo,
  promises,
}: {
  owner: string;
  repo: string;
  promises: ResourcePromises;
}) {
  const { number } = useParams<{
    owner: string;
    repo: string;
    number: string | undefined;
  }>();

  const questions = use(promises.questions);
  if (!questions) return null;

  return questions.map((question) => {
    const isActive = number === String(question.number);
    const author = question.author?.name;
    return (
      <Link
        key={question.id}
        href={`/${owner}/${repo}/questions/${question.number}`}
        className={`flex w-full border-b hover:bg-accent/50 select-none cursor-default py-2 px-2 ${
          isActive && "bg-sidebar"
        }`}
        prefetch={true}
        data-sidebar-item
        data-sidebar-item-active={isActive ? "true" : undefined}
      >
        <div className="flex flex-col w-full justify-start items-start min-w-0">
          <div className="text-sm truncate mb-0.5 w-full">{question.title}</div>
          <div className="text-xs text-muted-foreground flex items-center gap-1 w-full min-w-0">
            <span className="truncate min-w-0">{author}</span>
            <span className="shrink-0">•</span>
            <span className="shrink-0">
              {timeAgo(new Date(question.created_at))}
            </span>
          </div>
        </div>
      </Link>
    );
  });
}
