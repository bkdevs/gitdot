import type { ReviewAuthorResource, ReviewDiffResource } from "gitdot-api";
import { UserImage } from "@/(main)/[owner]/ui/user-image";
import { UserSlug } from "@/(main)/[owner]/ui/user-slug";
import { formatDateTime } from "@/util";

export function ReviewDiffHeader({
  diff,
  author,
}: {
  diff: ReviewDiffResource;
  author: ReviewAuthorResource | null;
}) {
  return (
    <div className="shrink-0 border-border border-b p-2">
      <div className="flex items-center gap-1.5 text-xs text-muted-foreground mb-1">
        <UserImage userId={author?.id} px={16} />
        {author && <UserSlug user={author} />}
        {author && <span>•</span>}
        <span>{formatDateTime(new Date(diff.created_at))}</span>
      </div>
      <div className="text-sm text-primary whitespace-pre-wrap break-words max-w-[80ch]">{diff.message}</div>
    </div>
  );
}
