import type {
  RepositoryBlobResource,
  RepositoryBlobsResource,
  RepositoryPathsResource,
} from "gitdot-api";
import type { RepoProvider } from "@/provider/types";

export const Resources = {
  readme: (p: RepoProvider) => p.getBlob("README.md"),
  paths: (p: RepoProvider) => p.getPaths(),
  blobs: (p: RepoProvider) => p.getBlobs(),
};

export interface Promises {
  readme: Promise<RepositoryBlobResource | null>;
  paths: Promise<RepositoryPathsResource | null>;
  blobs: Promise<RepositoryBlobsResource | null>;
}
