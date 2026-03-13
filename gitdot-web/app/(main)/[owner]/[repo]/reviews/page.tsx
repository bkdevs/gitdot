import { getReviews } from "@/dal";
import { ReviewsClient } from "./ui/reviews-client";

export default async function Page({
  params,
}: {
  params: Promise<{ owner: string; repo: string }>;
}) {
  const { owner, repo } = await params;
  const reviews = await getReviews(owner, repo);
  if (!reviews) return null;

  return <ReviewsClient owner={owner} repo={repo} reviews={reviews} />;
}
