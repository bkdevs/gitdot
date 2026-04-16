import type {
  RepositoryDiffFileResource,
  RepositoryDiffStatResource,
} from "gitdot-api";
import type { DiffData } from "@/actions";
import { ReviewDiffFileBody } from "./review-diff-file-body";
import { ReviewDiffFileHeader } from "./review-diff-file-header";

export function ReviewDiffFile({
  diff,
  data,
}: {
  diff: RepositoryDiffStatResource | RepositoryDiffFileResource;
  data: DiffData;
}) {
  return (
    <div
      data-diff-file
      className="rounded-sm border border-border overflow-hidden"
    >
      <ReviewDiffFileHeader diff={diff} />
      <ReviewDiffFileBody data={data} />
    </div>
  );
}
