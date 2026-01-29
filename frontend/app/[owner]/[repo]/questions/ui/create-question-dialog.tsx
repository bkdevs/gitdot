"use client";

import { useActionState, useState } from "react";
import {
  type CreateQuestionActionResult,
  createQuestionAction,
} from "@/actions";
import { Dialog, DialogContent, DialogTitle } from "@/ui/dialog";

export function CreateQuestionDialog({
  open,
  setOpen,
  owner,
  repo,
}: {
  open: boolean;
  setOpen: (open: boolean) => void;
  owner: string;
  repo: string;
}) {
  const [title, setTitle] = useState("");
  const [body, setBody] = useState("");

  const createQuestion = createQuestionAction.bind(null, owner, repo);
  const [state, formAction, isPending] = useActionState(
    async (_prev: CreateQuestionActionResult | null, formData: FormData) => {
      const result = await createQuestion(formData);
      if ("question" in result) {
        setOpen(false);
        setTitle("");
        setBody("");
      }
      return result;
    },
    null,
  );

  const isValid = title.trim() !== "" && body.trim() !== "";

  return (
    <Dialog open={open} onOpenChange={setOpen}>
      <DialogContent
        className="max-w-2xl min-w-2xl border-border rounded-md shadow-2xl top-[35%] p-4"
        animations={true}
        showOverlay={true}
      >
        <DialogTitle>
          <div className="flex flex-row w-full items-center justify-between">
            <span>Ask a question</span>
            <span className="text-xs font-normal text-muted-foreground">
              {owner}/{repo}
            </span>
          </div>
        </DialogTitle>
        <form action={formAction} className="flex flex-col gap-1">
          <input
            type="text"
            name="title"
            placeholder="Title..."
            value={title}
            onChange={(e) => setTitle(e.target.value)}
            className="w-full px-0 bg-background outline-none border-none"
            disabled={isPending}
            autoFocus
          />
          <textarea
            name="body"
            placeholder="Description..."
            value={body}
            onChange={(e) => setBody(e.target.value)}
            className="w-full px-0 text-sm bg-background outline-none border-none resize-none min-h-32"
            disabled={isPending}
          />
          {state && "error" in state && (
            <p className="text-xs text-red-500">{state.error}</p>
          )}
          <div className="flex justify-end">
            <button
              type="submit"
              disabled={!isValid || isPending}
              className="px-3 py-1.5 rounded-xs text-xs bg-primary text-primary-foreground disabled:opacity-50 disabled:cursor-not-allowed"
            >
              {isPending ? "Creating..." : "Ask"}
            </button>
          </div>
        </form>
      </DialogContent>
    </Dialog>
  );
}
