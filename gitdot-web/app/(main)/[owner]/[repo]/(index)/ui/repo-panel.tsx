"use client";

import type {
  GetRepositoryActivityResponse,
  RepositoryResource,
  UserResource,
} from "gitdot-api";
import { useState } from "react";
import { toast } from "@/(main)/context/toaster";
import { useUserContext } from "@/(main)/context/user";
import { useRightSidebar } from "@/(main)/hooks/use-sidebar";
import {
  starRepositoryAction,
  unstarRepositoryAction,
} from "@/actions/repository";
import { RepoActions } from "./repo-actions";
import { RepoActivity } from "./repo-activity";
import { RepoInfo } from "./repo-info";

export function RepoPanel({
  repository,
  activityPromise,
  currentUser,
  isAdmin,
}: {
  repository: RepositoryResource;
  activityPromise: Promise<GetRepositoryActivityResponse | null>;
  currentUser: UserResource | null;
  isAdmin: boolean;
}) {
  const open = useRightSidebar();
  const { openAuthDialog } = useUserContext();
  const [starred, setStarred] = useState(repository.user_star);

  const toggleStar = async () => {
    if (openAuthDialog()) return;
    const next = !starred;
    setStarred(next);
    const result = next
      ? await starRepositoryAction(repository.owner, repository.name)
      : await unstarRepositoryAction(repository.owner, repository.name);
    if ("error" in result) {
      setStarred(!next);
      toast(result.error);
    }
  };

  if (!open) return null;

  return (
    <div className="w-64 shrink-0 h-full border-l flex flex-col">
      <RepoInfo repository={repository} isAdmin={isAdmin} />
      <RepoActions
        repository={repository}
        starred={starred}
        toggleStar={toggleStar}
        isAdmin={isAdmin}
      />
      <RepoActivity
        activityPromise={activityPromise}
        starred={starred}
        currentUser={currentUser}
      />
    </div>
  );
}
