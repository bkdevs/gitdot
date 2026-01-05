import { getSession } from "@/lib/supabase";
import SignoutButton from "./ui/signout-button";

export default async function HomePage() {
  const session = await getSession();

  return (
    <div className="max-w-3xl mx-auto flex flex-col gap-4 items-center justify-center h-screen">
      <p>Build something great. </p>
      {session && <SignoutButton />}
    </div>
  );
}
