import { getReview } from "@/dal";
import { timeAgo } from "@/util";

export default async function Page({
  params,
}: {
  params: Promise<{ owner: string; repo: string; number: number }>;
}) {
  const { owner, repo, number } = await params;
  const review = await getReview(owner, repo, number);
  if (!review) return null;

  return (
    <div className="w-full">
      <div className="flex flex-col flex-1 min-w-0 pb-20">
        <div className="max-w-4xl pt-4 px-4 border-border border-r">
          <div className="flex flex-col gap-1">
            <h1 className="text-lg font-medium">
              {review.title || `Review #${review.number}`}
            </h1>
            <div className="flex flex-row gap-2 text-sm text-muted-foreground">
              <span>{review.status}</span>
              <span>•</span>
              <span>{review.author?.name}</span>
              <span>•</span>
              <span>{review.target_branch}</span>
              <span>•</span>
              <span>{timeAgo(new Date(review.created_at))}</span>
            </div>
            {review.description && (
              <p className="text-sm mt-2">{review.description}</p>
            )}
          </div>
        </div>

        <div className="w-full border-border border-b mt-4" />

        <div className="max-w-4xl border-border border-r">
          <div className="px-4 py-2 text-sm text-muted-foreground">
            {review.diffs.length} {review.diffs.length === 1 ? "diff" : "diffs"}
          </div>
        </div>

        <div className="flex flex-col max-w-4xl">
          {review.diffs.map((diff) => (
            <div
              key={diff.id}
              className="flex flex-col gap-1 px-4 py-3 border-border border-b border-r"
            >
              <div className="flex flex-row items-center gap-2">
                <span className="text-xs text-muted-foreground">
                  #{diff.position}
                </span>
                <span className="text-sm">{diff.title}</span>
                <span className="text-xs text-muted-foreground ml-auto">
                  {diff.status}
                </span>
              </div>
              {diff.description && (
                <p className="text-xs text-muted-foreground">
                  {diff.description}
                </p>
              )}
              {diff.revisions.length > 0 && (
                <div className="text-xs text-muted-foreground">
                  {diff.revisions.length}{" "}
                  {diff.revisions.length === 1 ? "revision" : "revisions"}
                  {" • "}
                  {diff.revisions[0].commit_hash.slice(0, 7)}
                </div>
              )}
            </div>
          ))}
        </div>
      </div>
    </div>
  );
}
