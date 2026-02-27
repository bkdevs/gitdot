import { getCurrentUser, listRunners } from "@/dal";
import { Runners } from "./ui/runners";

export default async function Page() {
  const user = await getCurrentUser();
  if (!user) return null;

  const runners = await listRunners(user.name);
  if (!runners) return null;

  return (
    <div className="flex flex-col w-full">
      <Runners runners={runners} />
    </div>
  );
}
