"use client";

import type { RepositoryBlobResource } from "gitdot-api";
import { createContext, Suspense, use, useContext, useMemo } from "react";
import { IdbProvider } from "@/provider";
import type { RepoProvider } from "@/provider/types";
import { firstNonNull } from "@/util";
import { MarkdownBody } from "./ui/markdown/markdown-body";
import { Resources, Promises } from "./resources";

const Context = createContext<Promises | null>(null);

export function Client({
  owner,
  repo,
  serverPromises,
}: {
  owner: string;
  repo: string;
  serverPromises: Promises;
}) {
  const idbPromises = useMemo(
    () => new IdbProvider(owner, repo).fetch(Resources),
    [owner, repo],
  );

  const context = useMemo(
    () => ({
      readme: firstNonNull(idbPromises.readme, serverPromises.readme),
    }),
    [idbPromises, serverPromises],
  );

  return (
    <Context value={context}>
      <Suspense>
        <ClientInner />
      </Suspense>
    </Context>
  );
}

function ClientInner() {
  const { readme } = useContext(Context)!;
  const resolved = use(readme);

  if (!resolved || resolved.type !== "file") {
    return <div className="p-2 text-sm">README.md not found</div>;
  }
  return (
    <div className="p-4 max-w-4xl">
      <MarkdownBody content={resolved.content} />
    </div>
  );
}
