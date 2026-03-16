import type { RepoProvider } from "@/provider/types";
import type { RepositoryBlobResource } from "gitdot-api";

export const Resources = {
  readme: (p: RepoProvider) => p.getBlob("README.md"),
};

export interface Promises {
  readme: Promise<RepositoryBlobResource | null>;
}
