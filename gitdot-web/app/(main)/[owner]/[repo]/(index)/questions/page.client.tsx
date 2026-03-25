"use client";

import { Suspense, use, useMemo, useState } from "react";
import {
  type ResourcePromisesType,
  type ResourceRequestsType,
  useResolvePromises,
} from "@/(main)/[owner]/[repo]/resources";
import { Loading } from "@/ui/loading";
import type { Resources } from "./page";
import { QuestionRow } from "./ui/question-row";
import { QuestionsHeader } from "./ui/questions-header";
import { processQuestions } from "./util";

type ResourceRequests = ResourceRequestsType<Resources>;
type ResourcePromises = ResourcePromisesType<Resources>;

export type QuestionsFilter = "popular" | "unanswered" | "all";
export type QuestionsSort =
  | "created-asc"
  | "created-desc"
  | "updated-asc"
  | "updated-desc"
  | "vote-asc"
  | "vote-desc";

export function PageClient({
  owner,
  repo,
  requests,
  promises,
}: {
  owner: string;
  repo: string;
  requests: ResourceRequests;
  promises: ResourcePromises;
}) {
  const resolvedPromises = useResolvePromises(owner, repo, requests, promises);
  return (
    <Suspense fallback={<Loading />}>
      <PageContent owner={owner} repo={repo} promises={resolvedPromises} />
    </Suspense>
  );
}

function PageContent({
  owner,
  repo,
  promises,
}: {
  owner: string;
  repo: string;
  promises: ResourcePromises;
}) {
  const questions = use(promises.questions);
  const [filter, setFilter] = useState<QuestionsFilter>("popular");
  const [sort, setSort] = useState<QuestionsSort>("created-asc");

  const processedQuestions = useMemo(
    () => processQuestions(questions ?? [], filter, sort),
    [questions, filter, sort],
  );

  if (!questions) return null;

  return (
    <div className="flex flex-col">
      <QuestionsHeader
        owner={owner}
        repo={repo}
        filter={filter}
        setFilter={setFilter}
        sort={sort}
        setSort={setSort}
      />
      {processedQuestions.map((question) => (
        <QuestionRow
          key={question.id}
          owner={owner}
          repo={repo}
          question={question}
        />
      ))}
    </div>
  );
}
