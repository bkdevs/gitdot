"use client";

import { MoreHorizontal, MoreVertical } from "lucide-react";
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
            className="opacity-0 group-hover:opacity-100 data-[state=open]:opacity-100 transition-opacity ring-0! outline-0!"
          >
            <MoreHorizontal className="size-4" />
          </Button>
        </DropdownMenuTrigger>
        <DropdownMenuContent align="end" className="rounded-none min-w-32 p-0">
          <DropdownMenuItem
            onClick={() => setEditing(true)}
            className="rounded-none px-2 py-1.5 text-xs cursor-pointer"
          >
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
