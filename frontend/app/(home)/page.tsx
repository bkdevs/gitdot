import { getClaims } from "@/lib/supabase";
import CreateRepoButton from "./ui/create-repo-button";
import SignoutButton from "./ui/signout-button";

export default async function HomePage() {
  const claims = await getClaims();

  return (
    <div className="max-w-3xl mx-auto flex flex-col gap-4 items-center justify-center h-screen">
      <p>Build something great. </p>
      {claims && (
        <>
          <CreateRepoButton />
          <SignoutButton />
        </>
      )}
    </div>
  );
}
