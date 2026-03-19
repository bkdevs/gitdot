"use client";

import { useParams } from "next/navigation";
import { Suspense, use } from "react";
import Link from "@/ui/link";
import { timeAgoFull } from "@/util";
import { useRepoContext } from "../context";
import { CommitsHeader } from "./ui/commits-header";
import { CommitsList } from "./ui/commits-list";
import { CommitsGrid } from "./ui/commits-grid";

export function CommitsClient() {
  const { owner, repo } = useParams<{ owner: string; repo: string }>();

  const commits = use(useRepoContext().commits);
  if (!commits) return null;

  return (
    <div className="flex flex-col">
      <CommitsHeader />
      <CommitsGrid commits={commits} />
      <CommitsList commits={commits} />
    </div>
  );
}

export default function Page() {
  return (
    <Suspense>
      <CommitsClient />
    </Suspense>
  );
}
