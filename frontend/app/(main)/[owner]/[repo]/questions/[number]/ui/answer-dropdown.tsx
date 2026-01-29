"use client";

import { MoreHorizontal } from "lucide-react";
import { useState } from "react";
import type { AnswerResponse } from "@/lib/dto";
import { Button } from "@/ui/button";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/ui/dropdown-menu";
import { EditAnswerDialog } from "./edit-answer-dialog";

export const AnswerDropdown = ({
  owner,
  repo,
  number,
  answer,
}: {
  owner: string;
  repo: string;
  number: number;
  answer: AnswerResponse;
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
      <EditAnswerDialog
        owner={owner}
        repo={repo}
        number={number}
        answer={answer}
        open={editing}
        setOpen={setEditing}
      />
    </>
  );
};
