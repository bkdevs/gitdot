import type { ReviewCommentResource } from "gitdot-api";
import { UserImage } from "@/(main)/[owner]/ui/user-image";

export function ReviewDiffFileBubbles({
  commentThreads,
  userId,
}: {
  commentThreads: Array<{ top: number; comments: ReviewCommentResource[] }>;
  userId: string | undefined;
}) {
  if (commentThreads.length === 0) return null;

  return (
    <>
      {commentThreads.map((thread) => (
        <div
          key={thread.top}
          className="absolute z-50 flex flex-row items-center gap-1.5 left-full ml-2 px-2 py-0.5 bg-background border border-border rounded-full animate-in fade-in duration-200"
          style={{ top: thread.top }}
        >
          <UserImage userId={userId} px={16} />
          <span className="text-xs font-sans text-muted-foreground">
            {thread.comments.length}
          </span>
        </div>
      ))}
    </>
  );
}
