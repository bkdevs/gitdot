import type { QuestionResource } from "gitdot-api";
import { fetchResources } from "@/provider/server";
import { PageClient } from "./page.client";

export type Resources = {
  questions: QuestionResource[] | null;
};

export default async function Page({
  params,
}: {
  params: Promise<{ owner: string; repo: string }>;
}) {
  const { owner, repo } = await params;
  const { requests, promises } = fetchResources(owner, repo, {
    questions: (p) => p.getQuestions(),
  });
  return (
    <PageClient
      owner={owner}
      repo={repo}
      requests={requests}
      promises={promises}
    />
  );
}
