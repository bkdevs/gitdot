"use client";

import { useActionState, useOptimistic, useState } from "react";
import { MoreHorizontal } from "lucide-react";
import { MarkdownBody } from "@/[owner]/[repo]/ui/markdown/markdown-body";
import type { QuestionResponse } from "@/lib/dto/question";
import { formatDate, timeAgoFull } from "@/util";
import { Comments } from "./comments";
import { VoteBox } from "./vote-box";
import { updateQuestionAction } from "@/actions";
import { Button } from "@/ui/button";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/ui/dropdown-menu";
import { Dialog, DialogContent, DialogTitle } from "@/ui/dialog";

type QuestionCardProps = {
  question: QuestionResponse;
  owner: string;
  repo: string;
};

export function QuestionCard({ question, owner, repo }: QuestionCardProps) {
  const wasUpdated = question.created_at !== question.updated_at;

  const [isEditing, setIsEditing] = useState(false);
  const [editTitle, setEditTitle] = useState(question.title);
  const [editBody, setEditBody] = useState(question.body);

  const [optimistic, setOptimistic] = useOptimistic(
    { title: question.title, body: question.body },
    (_state, newValue: { title: string; body: string }) => newValue,
  );

  const updateQuestion = updateQuestionAction.bind(null, owner, repo, question.number);
  const [state, formAction] = useActionState(
    async (_prevState: { error?: string } | null, formData: FormData) => {
      const title = formData.get("title") as string;
      const body = formData.get("body") as string;

      setOptimistic({ title, body });
      setIsEditing(false);

      const result = await updateQuestion(formData);
      if (result.success) {
        return null;
      }
      return { error: result.error };
    },
    null,
  );

  const handleCancel = () => {
    setEditTitle(optimistic.title);
    setEditBody(optimistic.body);
    setIsEditing(false);
  };

  return (
    <div className="flex group">
      <VoteBox
        targetType="question"
        owner={owner}
        repo={repo}
        number={question.number}
        score={question.upvote}
        userVote={question.user_vote}
      />
      <div className="flex-1">
        <div className="flex items-start justify-between">
          <h1 className="text-xl font-medium">{optimistic.title}</h1>
          <DropdownMenu>
            <DropdownMenuTrigger asChild>
              <Button
                variant="ghost"
                size="icon-sm"
                className="opacity-0 group-hover:opacity-100 transition-opacity"
              >
                <MoreHorizontal className="size-4" />
              </Button>
            </DropdownMenuTrigger>
            <DropdownMenuContent align="end">
              <DropdownMenuItem onClick={() => {
                  setEditTitle(optimistic.title);
                  setEditBody(optimistic.body);
                  setIsEditing(true);
                }}>
                Edit
              </DropdownMenuItem>
            </DropdownMenuContent>
          </DropdownMenu>
        </div>
        <MarkdownBody content={optimistic.body} />

        <Dialog open={isEditing} onOpenChange={setIsEditing}>
          <DialogContent>
            <DialogTitle>Edit Question</DialogTitle>
            <form action={formAction}>
              <input
                name="title"
                value={editTitle}
                onChange={(e) => setEditTitle(e.target.value)}
                className="w-full text-xl font-medium bg-transparent border-b border-input outline-none py-1"
              />
              <textarea
                name="body"
                value={editBody}
                onChange={(e) => setEditBody(e.target.value)}
                className="w-full h-48 mt-2 border rounded-xs resize-none text-sm focus:outline-none focus:ring-1 focus:ring-ring p-2"
              />
              {state?.error && (
                <p className="text-xs text-red-500 mt-1">{state.error}</p>
              )}
              <div className="flex justify-end gap-2 mt-2">
                <button
                  type="button"
                  onClick={handleCancel}
                  className="h-7 px-4 text-sm font-medium border bg-background hover:bg-accent hover:text-accent-foreground"
                >
                  Cancel
                </button>
                <button
                  type="submit"
                  className="h-7 px-4 text-sm font-medium bg-primary text-primary-foreground hover:bg-primary/90"
                >
                  Save
                </button>
              </div>
            </form>
          </DialogContent>
        </Dialog>

        <div className="flex flex-row gap-1 items-center text-xs text-muted-foreground">
          <span className="text-blue-400 cursor-pointer">
            {question.author.name}
          </span>
          <span>
            <span className="text-muted-foreground">asked</span>{" "}
            {formatDate(new Date(question.created_at))}
            {", "}
            {wasUpdated ? (
              <>
                <span className="text-muted-foreground">updated</span>{" "}
                {timeAgoFull(new Date(question.updated_at))}
              </>
            ) : (
              timeAgoFull(new Date(question.created_at))
            )}
          </span>
        </div>

        <Comments
          parentType="question"
          owner={owner}
          repo={repo}
          number={question.number}
          comments={question.comments}
        />
      </div>
    </div>
  );
}
