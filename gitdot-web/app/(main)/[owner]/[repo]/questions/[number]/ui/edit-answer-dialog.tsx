"use client";

import { VisuallyHidden } from "@radix-ui/react-visually-hidden";
import type { AnswerResource } from "gitdot-api";
import { useActionState, useState } from "react";
import { type UpdateAnswerActionResult, updateAnswerAction } from "@/actions";
import { Dialog, DialogContent, DialogTitle } from "@/ui/dialog";

export function EditAnswerDialog({
  owner,
  repo,
  number,
  answer,
  open,
  setOpen,
}: {
  owner: string;
  repo: string;
  number: number;
  answer: AnswerResource;
  open: boolean;
  setOpen: (open: boolean) => void;
}) {
  const [body, setBody] = useState(answer.body);

  const updateAnswer = updateAnswerAction.bind(
    null,
    owner,
    repo,
    number,
    answer.id,
  );

  const [state, formAction, isPending] = useActionState(
    async (_prevState: UpdateAnswerActionResult, formData: FormData) => {
      const result = await updateAnswer(formData);
      if ("answer" in result) {
        setOpen(false);
      }
      return result;
    },
    { answer: answer },
  );

  const isValid =
    body.trim() !== "" && "answer" in state && state.answer.body !== body;

  return (
    <Dialog open={open} onOpenChange={setOpen}>
      <DialogContent
        className="max-w-2xl min-w-2xl border-black rounded-xs shadow-2xl top-[35%] p-0 overflow-hidden"
        animations={true}
        showOverlay={true}
        onOpenAutoFocus={(e) => e.preventDefault()}
      >
        <VisuallyHidden>
          <DialogTitle title={"Edit answer"} />
        </VisuallyHidden>
        <form action={formAction} className="relative">
          <textarea
            name="body"
            placeholder="Write your answer..."
            value={body}
            onChange={(e) => setBody(e.target.value)}
            className="w-full p-2 text-sm bg-background outline-none resize-none min-h-32"
            disabled={isPending}
          />
          {"error" in state && (
            <p className="text-xs text-red-500 px-3 pb-2">{state.error}</p>
          )}
          <div className="flex items-center justify-between pl-2 py-2 border-t border-border h-9">
            <span className="text-xs text-muted-foreground">
              Edit answer in{" "}
              <span className="text-foreground">
                {owner}/{repo}
              </span>
            </span>
            <div>
              <button
                type="reset"
                className="px-3 py-1.5 h-9 text-xs border-b border-l border-r hover:bg-accent/50"
                onClick={() => setOpen(false)}
              >
                Cancel
              </button>
              <button
                type="submit"
                disabled={!isValid || isPending}
                className="px-3 py-1.5 h-9 text-xs bg-primary text-primary-foreground disabled:opacity-50 disabled:cursor-not-allowed"
              >
                {isPending ? "Saving..." : "Save"}
              </button>
            </div>
          </div>
        </form>
      </DialogContent>
    </Dialog>
  );
}
