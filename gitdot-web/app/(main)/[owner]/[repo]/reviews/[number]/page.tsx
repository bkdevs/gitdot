import { getReview } from "@/dal";
import { ReviewDetail } from "./review-detail";

export default async function Page({
  params,
}: {
  params: Promise<{ owner: string; repo: string; number: number }>;
}) {
  const { owner, repo, number } = await params;
  const review = await getReview(owner, repo, number);
  if (!review) return null;

  return (
    <ReviewDetail owner={owner} repo={repo} number={number} review={review} />
  );
}
