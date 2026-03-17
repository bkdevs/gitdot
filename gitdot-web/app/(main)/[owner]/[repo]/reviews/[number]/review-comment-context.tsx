"use client";

import { createContext, useContext } from "react";

export interface DraftComment {
  diff_id: string;
  revision_id: string | null;
  file_path: string;
  line_number: number;
  side: "old" | "new";
  author_name: string;
  body: string;
  created_at: string;
}

export interface CommentInput {
  filePath: string;
  lineNumber: number;
  side: "old" | "new";
}

interface ReviewCommentContextValue {
  comments: DraftComment[];
  addComment: (
    filePath: string,
    lineNumber: number,
    side: "old" | "new",
    body: string,
  ) => void;
  canComment: boolean;
  activeInput: CommentInput | null;
  setActiveInput: (input: CommentInput | null) => void;
}

export const ReviewCommentContext =
  createContext<ReviewCommentContextValue | null>(null);

export function useReviewComments() {
  return useContext(ReviewCommentContext);
}

const DiffFileContext = createContext<string | null>(null);

export function DiffFileProvider({
  filePath,
  children,
}: {
  filePath: string;
  children: React.ReactNode;
}) {
  return (
    <DiffFileContext.Provider value={filePath}>
      {children}
    </DiffFileContext.Provider>
  );
}

export function useDiffFile() {
  return useContext(DiffFileContext);
}
