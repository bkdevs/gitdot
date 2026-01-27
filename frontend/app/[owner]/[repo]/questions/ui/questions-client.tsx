"use client";

import { useState } from "react";
import type { QuestionResponse } from "@/lib/dto";
import { QuestionRow } from "./question-row";
import { QuestionsHeader } from "./questions-header";

export type QuestionsFilter = "popular" | "unanswered" | "all";
export type QuestionsSort =
  | "created-asc"
  | "created-desc"
  | "updated-asc"
  | "updated-desc"
  | "vote-asc"
  | "vote-desc";

export function QuestionsClient({
  owner,
  repo,
  questions,
}: {
  owner: string;
  repo: string;
  questions: QuestionResponse[];
}) {
  const [filter, setFilter] = useState<QuestionsFilter>("popular");
  const [sort, setSort] = useState<QuestionsSort>("created-asc");

  return (
    <div className="flex flex-col">
      <QuestionsHeader
        filter={filter}
        setFilter={setFilter}
        sort={sort}
        setSort={setSort}
      />
      {questions.map((question) => (
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
