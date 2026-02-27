import { Runners } from "@/(main)/settings/runners/ui/runners";
import { listRunners } from "@/dal";

export default async function Page({
  params,
}: {
  params: Promise<{ owner: string }>;
}) {
  const { owner } = await params;
  const runners = await listRunners(owner);
  if (!runners) return null;

  return (
    <div className="flex flex-col w-full">
      <Runners runners={runners} basePath={`/${owner}/settings/runners`} />
    </div>
  );
}
