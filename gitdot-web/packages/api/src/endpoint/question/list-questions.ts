import { z } from "zod";
import { QuestionResource } from "../../resource";
import type { Endpoint } from "../endpoint";

export const ListQuestionsRequest = z.object({});
export type ListQuestionsRequest = z.infer<typeof ListQuestionsRequest>;

export const ListQuestionsResponse = z.array(QuestionResource);
export type ListQuestionsResponse = z.infer<typeof ListQuestionsResponse>;

export const ListQuestions = {
  path: "/repository/{owner}/{repo}/questions",
  method: "GET",
  request: ListQuestionsRequest,
  response: ListQuestionsResponse,
} as const satisfies Endpoint;
export type ListQuestions = typeof ListQuestions;
