"use client";

import { Suspense, use } from "react";
import { useRepoContext } from "../context";
import { CommitsGrid } from "./ui/commits-grid";
import { CommitsHeader } from "./ui/commits-header";
import { CommitsList } from "./ui/commits-list";

export function CommitsClient() {
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
