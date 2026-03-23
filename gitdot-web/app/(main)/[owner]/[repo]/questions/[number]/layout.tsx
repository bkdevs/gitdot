import type { QuestionResource } from "gitdot-api";
import { fetchResources } from "@/provider/server";
import { LayoutClient } from "./layout.client";

export type Resources = {
  questions: QuestionResource[] | null;
};

export default async function Layout({
  params,
  children,
}: {
  params: Promise<{ owner: string; repo: string; number: string }>;
  children: React.ReactNode;
}) {
  const { owner, repo } = await params;
  const { requests, promises } = fetchResources(owner, repo, {
    questions: (p) => p.getQuestions(),
  });

  return (
    <LayoutClient
      owner={owner}
      repo={repo}
      requests={requests}
      promises={promises}
    >
      {children}
    </LayoutClient>
  );
}
