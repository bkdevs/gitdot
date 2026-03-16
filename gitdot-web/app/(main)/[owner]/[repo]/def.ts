import type { RepoProvider } from "@/provider/types";

export const repoDef = {
  readme: (p: RepoProvider) => p.getBlob("README.md"),
};
