"use client";

import { useUser } from "@/(main)/providers/user-provider";
import { type CreateAnswerActionResult, createAnswerAction } from "@/actions";
import { AnswerResponse } from "@/lib/dto";
import { Button } from "@/ui/button";
import { cn } from "@/util";
import { useActionState, useState } from "react";

export function AnswerForm({
  owner,
  repo,
  number,
  answers,
}: {
  owner: string;
  repo: string;
  number: number;
  answers: AnswerResponse[]
}) {
  const createAnswer = createAnswerAction.bind(null, owner, repo, number);
  const [body, setBody] = useState("");
  const [state, formAction, isPending] = useActionState(
    async (_prevState: CreateAnswerActionResult | null, formData: FormData) => {
      return await createAnswer(formData);
    },
    null,
  );
  const isValid = body.trim() !== "";
  const user = useUser();
  const answeredQuestion = answers.find(
    (answer) => answer.author_id === user?.id,
  );

  if (answeredQuestion) {
    return null;
  }

  return (
    <div className={cn("ml-3", answers.length > 0 && "mt-12")}>
      <form action={formAction}>
        <div className="relative">
          <textarea
            name="body"
            value={body}
            onChange={(e) => setBody(e.target.value)}
            className="w-full h-48 p-2 border rounded-xs resize-none text-sm focus:outline-none focus:ring-1 focus:ring-ring"
            placeholder="Write your answer..."
            disabled={isPending}
          />
          <div className="absolute bottom-1.5 right-0">
            <Button
              type="submit"
              disabled={!isValid || isPending}
              className="rounded-none"
            >
              {isPending ? "Submitting..." : "Submit"}
            </Button>
          </div>
        </div>
        {state && "error" in state && (
          <p className="text-xs text-red-500 mt-1">{state.error}</p>
        )}
      </form>
    </div>
  );
}
