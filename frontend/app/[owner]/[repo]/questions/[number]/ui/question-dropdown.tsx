"use client";

import { MoreHorizontal } from "lucide-react";
import { useState } from "react";
import type { QuestionResponse } from "@/lib/dto";
import { Button } from "@/ui/button";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/ui/dropdown-menu";
import { EditQuestionDialog } from "./edit-question-dialog";

export const QuestionDropdown = ({
  owner,
  repo,
  question,
}: {
  owner: string;
  repo: string;
  question: QuestionResponse;
}) => {
  const [editing, setEditing] = useState(false);

  return (
    <>
      <DropdownMenu>
        <DropdownMenuTrigger asChild>
          <Button
            variant="ghost"
            size="icon-sm"
            className="opacity-0 group-hover:opacity-100 transition-opacity"
          >
            <MoreHorizontal className="size-3" />
          </Button>
        </DropdownMenuTrigger>
        <DropdownMenuContent align="end">
          <DropdownMenuItem onClick={() => setEditing(true)}>
            Edit
          </DropdownMenuItem>
        </DropdownMenuContent>
      </DropdownMenu>
      <EditQuestionDialog
        owner={owner}
        repo={repo}
        question={question}
        open={editing}
        setOpen={setEditing}
      />
    </>
  );
};
