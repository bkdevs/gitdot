import { getCurrentUser } from "@/lib/dal";

export default async function Page() {
  await getCurrentUser();

  return <div className="p-2 text-sm">To come.</div>;
}
