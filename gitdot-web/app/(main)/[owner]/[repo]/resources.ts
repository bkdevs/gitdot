/// a separate shared types file to be used by both
///   - ssr: layout.tsx
///   - csr: context.tsxj

import type {
  RepositoryBlobsResource,
  RepositoryCommitResource,
  RepositoryPathsResource,
  RepositorySettingsResource,
} from "gitdot-api";
import type { RepoProvider } from "@/provider/types";

export const RepoResources = {
  paths: (p: RepoProvider) => p.getPaths(),
  commits: (p: RepoProvider) => p.getCommits(),
  blobs: (p: RepoProvider) => p.getBlobs(),
  settings: (p: RepoProvider) => p.getSettings(),
};

export interface RepoPromises {
  paths: Promise<RepositoryPathsResource | null>;
  commits: Promise<RepositoryCommitResource[] | null>;
  blobs: Promise<RepositoryBlobsResource | null>;
  settings: Promise<RepositorySettingsResource | null>;
}
