import { z } from "zod";
import { QuestionResource } from "../../resource";
import type { Endpoint } from "../endpoint";

export const GetQuestionsRequest = z.object({});
export type GetQuestionsRequest = z.infer<typeof GetQuestionsRequest>;

export const GetQuestionsResponse = z.array(QuestionResource);
export type GetQuestionsResponse = z.infer<typeof GetQuestionsResponse>;

export const GetQuestions = {
  path: "/repository/{owner}/{repo}/questions",
  method: "GET",
  request: GetQuestionsRequest,
  response: GetQuestionsResponse,
} satisfies Endpoint;
export type GetQuestions = typeof GetQuestions;
