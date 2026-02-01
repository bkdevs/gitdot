import { getCurrentUser } from "@/lib/dal";

export default async function Page() {
  const user = await getCurrentUser();
  return user ? <div>has user</div> : <div>something</div>;
}
