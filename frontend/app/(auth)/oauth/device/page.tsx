import { redirect } from "next/navigation";
import { getCurrentUser } from "@/lib/dal";
import AuthorizeDeviceForm from "../../ui/authorize-device-form";

export default async function Page() {
  const user = await getCurrentUser();
  if (!user) {
    redirect("/login?redirect=/oauth/device");
  }

  return (
    <div className="max-w-3xl mx-auto flex gap-4 items-center justify-center h-screen">
      <AuthorizeDeviceForm />
    </div>
  );
}
