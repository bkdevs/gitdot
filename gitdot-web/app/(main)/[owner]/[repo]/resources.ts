import type {
  RepositoryBlobsResource,
  RepositoryCommitResource,
  RepositoryPathsResource,
} from "gitdot-api";
import type { RepoProvider } from "@/provider/types";

export const Resources = {
  paths: (p: RepoProvider) => p.getPaths(),
  commits: (p: RepoProvider) => p.getCommits(),
  blobs: (p: RepoProvider) => p.getBlobs(),
};

export interface Promises {
  paths: Promise<RepositoryPathsResource | null>;
  commits: Promise<RepositoryCommitResource[] | null>;
  blobs: Promise<RepositoryBlobsResource | null>;
}
