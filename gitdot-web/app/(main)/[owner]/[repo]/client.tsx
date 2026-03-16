"use client";

import type { RepositoryBlobResource } from "gitdot-api";
import { createContext, useContext, useEffect, useState } from "react";
import { IdbProvider } from "@/provider";
import type { RepoProvider } from "@/provider/types";
import { firstNonNull } from "@/util";
import { MarkdownBody } from "./ui/markdown/markdown-body";

export const Resources = {
  readme: (p: RepoProvider) => p.getBlob("README.md"),
};
interface Promises {
  readme: Promise<RepositoryBlobResource | null>;
}
interface Context {
  readme: RepositoryBlobResource | null | undefined;
}
const Context = createContext<Context | null>(null);

export function Client({
  owner,
  repo,
  serverPromises,
}: {
  owner: string;
  repo: string;
  serverPromises: Promises;
}) {
  const [readme, setReadme] = useState<
    RepositoryBlobResource | null | undefined
  >(undefined);

  useEffect(() => {
    const idbPromises = new IdbProvider(owner, repo).fetch(Resources);

    firstNonNull(idbPromises.readme, serverPromises.readme)
      .then(setReadme)
      .catch(() => setReadme(null));
  }, [owner, repo, serverPromises]);

  return (
    <Context value={{ readme }}>
      <ClientInner />
    </Context>
  );
}

function ClientInner() {
  const { readme } = useContext(Context)!;

  if (readme === undefined) return null;
  if (readme === null || readme.type !== "file") {
    return <div className="p-2 text-sm">README.md not found</div>;
  }
  return (
    <div className="p-4 max-w-4xl">
      <MarkdownBody content={readme.content} />
    </div>
  );
}
