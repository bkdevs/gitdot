"use client";

import { VisuallyHidden } from "@radix-ui/react-visually-hidden";
import type { QuestionResource } from "gitdot-api";
import { useActionState, useState } from "react";
import {
  type UpdateQuestionActionResult,
  updateQuestionAction,
} from "@/actions";
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
  question: QuestionResource;
  open: boolean;
  setOpen: (open: boolean) => void;
}) {
  const [title, setTitle] = useState(question.title);
  const [body, setBody] = useState(question.body);

  const updateQuestion = updateQuestionAction.bind(
    null,
    owner,
    repo,
    question.number,
  );

  const [state, formAction, isPending] = useActionState(
    async (_prevState: UpdateQuestionActionResult, formData: FormData) => {
      const result = await updateQuestion(formData);
      if ("question" in result) {
        setOpen(false);
      }
      return result;
    },
    { question: question },
  );

  const isValid =
    title.trim() !== "" &&
    body.trim() !== "" &&
    "question" in state &&
    (state.question.title !== title || state.question.body !== body);

  return (
    <Dialog open={open} onOpenChange={setOpen}>
      <DialogContent
        className="max-w-2xl min-w-2xl border-black rounded-xs shadow-2xl top-[35%] p-0 overflow-hidden"
        animations={true}
        showOverlay={true}
        onOpenAutoFocus={(e) => e.preventDefault()}
      >
        <VisuallyHidden>
          <DialogTitle title={"Edit question"} />
        </VisuallyHidden>
        <form action={formAction} className="relative">
          <input
            type="text"
            name="title"
            placeholder="Question title..."
            value={title}
            onChange={(e) => setTitle(e.target.value)}
            className="w-full p-2 bg-background outline-none border-b border-border"
            disabled={isPending}
          />
          <textarea
            name="body"
            placeholder="Write a description..."
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
              Edit question in{" "}
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
