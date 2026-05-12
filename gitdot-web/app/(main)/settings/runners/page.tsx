import { getCurrentUser, listRunners } from "@/dal";
import { Runners } from "./ui/runners";

export default async function Page() {
  const current = await getCurrentUser();
  if (!current) return null;
  const { user } = current;

  const runners = await listRunners(user.name);
  if (!runners) return null;

  return (
    <div className="flex flex-col w-full">
      <Runners runners={runners} basePath="/settings/runners" />
    </div>
  );
}
