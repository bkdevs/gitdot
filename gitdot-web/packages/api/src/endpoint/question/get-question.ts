import { z } from "zod";
import { QuestionResource } from "../../resource";
import type { Endpoint } from "../endpoint";

export const GetQuestionRequest = z.object({});
export type GetQuestionRequest = z.infer<typeof GetQuestionRequest>;

export const GetQuestionResponse = QuestionResource;
export type GetQuestionResponse = z.infer<typeof GetQuestionResponse>;

export const GetQuestion = {
  path: "/repository/{owner}/{repo}/question/{number}",
  method: "GET",
  request: GetQuestionRequest,
  response: GetQuestionResponse,
} as const satisfies Endpoint;
export type GetQuestion = typeof GetQuestion;
