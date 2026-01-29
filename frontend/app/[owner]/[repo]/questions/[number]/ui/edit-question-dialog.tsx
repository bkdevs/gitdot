"use client";

import { useActionState } from "react";
import {
  type UpdateQuestionActionResult,
  updateQuestionAction,
} from "@/actions";
import type { QuestionResponse } from "@/lib/dto/question";
import { Dialog, DialogContent, DialogTitle } from "@/ui/dialog";

export function EditQuestionDialog({
  owner,
  repo,
  question,
  open,
  setOpen,
}: {
  owner: string;
  repo: string;
  question: QuestionResponse;
  open: boolean;
  setOpen: (open: boolean) => void;
}) {
  const updateQuestion = updateQuestionAction.bind(
    null,
    owner,
    repo,
    question.number,
  );

  const [state, formAction, isPending] = useActionState(
    async (_prevState: UpdateQuestionActionResult, formData: FormData) => {
      setOpen(false);

      const result = await updateQuestion(formData);
      return result;
    },
    { question: question },
  );

  return (
    <Dialog open={open} onOpenChange={setOpen}>
      <DialogContent
        className="max-w-2xl min-w-2xl border-border rounded-md shadow-2xl top-[35%] p-4"
        animations={true}
        showOverlay={true}
        onOpenAutoFocus={(e) => e.preventDefault()}
      >
        <DialogTitle>
          <div className="flex flex-row w-full items-center justify-between">
            <span>Edit question</span>
            <span className="text-xs font-normal text-muted-foreground">
              {owner}/{repo}
            </span>
          </div>
        </DialogTitle>
        <form action={formAction} className="flex flex-col gap-1">
          <input
            name="title"
            defaultValue={question.title}
            className="w-full px-0 bg-background outline-none border-none"
            disabled={isPending}
          />
          <textarea
            name="body"
            defaultValue={question.body}
            className="w-full px-0 text-sm bg-background outline-none border-none resize-none min-h-32"
            disabled={isPending}
          />
          {"error" in state && (
            <p className="text-xs text-red-500">{state.error}</p>
          )}
          <div className="flex justify-end">
            <button
              type="submit"
              disabled={isPending}
              className="px-3 py-1.5 rounded-xs text-xs bg-primary text-primary-foreground disabled:opacity-50 disabled:cursor-not-allowed"
            >
              {isPending ? "Saving..." : "Save"}
            </button>
          </div>
        </form>
      </DialogContent>
    </Dialog>
  );
}
