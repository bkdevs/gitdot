"use client";

import { useActionState, useState } from "react";
import { useAuthBlocker } from "@/(main)/providers/auth-blocker-provider";
import { type CreateAnswerActionResult, createAnswerAction } from "@/actions";
import { Button } from "@/ui/button";

export function AnswerForm({
  owner,
  repo,
  number,
}: {
  owner: string;
  repo: string;
  number: number;
}) {
  const { requireAuth } = useAuthBlocker();
  const [body, setBody] = useState("");

  const createAnswer = createAnswerAction.bind(null, owner, repo, number);
  const [state, formAction, isPending] = useActionState(
    async (_prevState: CreateAnswerActionResult | null, formData: FormData) => {
      if (requireAuth()) return null;
      return await createAnswer(formData);
    },
    null,
  );

  const isValid = body.trim() !== "";

  return (
    <div>
      <form action={formAction}>
        <div className="relative">
          <textarea
            name="body"
            value={body}
            onChange={(e) => setBody(e.target.value)}
            className="w-full h-48 p-2 border border-b-border border-r-border border-t-transparent border-l-transparent rounded-xs resize-none text-sm focus:outline-none focus:border-black transition-colors duration-200"
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
