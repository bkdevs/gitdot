"use client";

import { useActionState, useState } from "react";
import { createAnswerAction } from "@/actions";
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
  const [body, setBody] = useState("");

  const [state, formAction, isPending] = useActionState(
    async (_prevState: { error?: string } | null, formData: FormData) => {
      const result = await createAnswerAction(formData);
      if (result.success) {
        setBody("");
        return null;
      }
      return { error: result.error };
    },
    null,
  );

  const isValid = body.trim() !== "";

  return (
    <div className="ml-3 mt-12">
      <form action={formAction}>
        <input type="hidden" name="owner" value={owner} />
        <input type="hidden" name="repo" value={repo} />
        <input type="hidden" name="number" value={number} />
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
        {state?.error && (
          <p className="text-xs text-red-500 mt-1">{state.error}</p>
        )}
      </form>
    </div>
  );
}
