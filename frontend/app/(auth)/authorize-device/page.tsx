import { redirect } from "next/navigation";
import { getSession } from "@/lib/supabase";
import AuthorizeForm from "./ui/authorize-form";

interface AuthorizeDevicePageProps {
  searchParams: Promise<{ code?: string }>;
}

export default async function AuthorizeDevicePage({
  searchParams,
}: AuthorizeDevicePageProps) {
  const session = await getSession();

  if (!session) {
    const params = await searchParams;
    const code = params.code;
    const redirectUrl = code
      ? `/authorize-device?code=${code}`
      : "/authorize-device";
    redirect(`/login?redirect=${encodeURIComponent(redirectUrl)}`);
  }

  const params = await searchParams;
  const userCode = params.code;

  return (
    <div className="max-w-md mx-auto flex flex-col gap-6 items-center justify-center h-screen">
      <div className="text-center">
        <h1 className="text-2xl font-bold mb-2">Authorize Device</h1>
        <p className="text-gray-600">
          A CLI application is requesting access to your account.
        </p>
      </div>
      <AuthorizeForm initialCode={userCode} />
    </div>
  );
}
