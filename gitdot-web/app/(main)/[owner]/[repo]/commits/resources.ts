import type { RepositoryCommitResource } from "gitdot-api";
import type { RepoProvider } from "@/provider/types";

export const Resources = {
  commits: (p: RepoProvider) => p.getCommits(),
};

export interface Promises {
  commits: Promise<RepositoryCommitResource[] | null>;
}
