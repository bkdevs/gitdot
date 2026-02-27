"use client";

import type { AnswerResource } from "gitdot-api-ts";
import { MoreHorizontal } from "lucide-react";
import { useState } from "react";
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
  answer: AnswerResource;
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
