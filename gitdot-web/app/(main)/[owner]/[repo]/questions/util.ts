import type { QuestionResource } from "gitdot-api-ts";
import type { QuestionsFilter, QuestionsSort } from "./ui/questions-client";

export function processQuestions(
  questions: QuestionResource[],
  filter: QuestionsFilter,
  sort: QuestionsSort,
) {
  let result: QuestionResource[];
  switch (filter) {
    case "unanswered":
      result = questions.filter((q) => q.answers.length === 0);
      break;
    default:
      result = questions;
      break;
  }

  return [...result].sort((a, b) => {
    switch (sort) {
      case "created-asc":
        return (
          new Date(a.created_at).getTime() - new Date(b.created_at).getTime()
        );
      case "created-desc":
        return (
          new Date(b.created_at).getTime() - new Date(a.created_at).getTime()
        );
      case "updated-asc":
        return (
          new Date(a.updated_at).getTime() - new Date(b.updated_at).getTime()
        );
      case "updated-desc":
        return (
          new Date(b.updated_at).getTime() - new Date(a.updated_at).getTime()
        );
      case "vote-asc":
        return a.upvote - b.upvote;
      case "vote-desc":
        return b.upvote - a.upvote;
      default:
        return 0;
    }
  });
}
