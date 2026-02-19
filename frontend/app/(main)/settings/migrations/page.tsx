import { getCurrentUser } from "@/lib/dal";
import { Migrations } from "./ui/migrations";

export default async function Page() {
  const user = await getCurrentUser();
  if (!user) return null;

  return (
    <div className="flex flex-col w-full">
      <Migrations />
    </div>
  );
}
