import { redirect } from "next/navigation";
import type { NextRequest } from "next/server";
import { createInstallation } from "@/lib/dal";

export async function GET(request: NextRequest) {
  const { searchParams } = new URL(request.url);
  const installationId = searchParams.get("installation_id");
  if (!installationId) {
    redirect("/home");
    return;
  }

  const installation = await createInstallation(Number(installationId));
  if (!installation) {
    redirect("/home");
    return;
  }

  const state = searchParams.get("state");
  if (!state) {
    redirect("/home");
    return;
  }

  if (state === "onboarding") {
    redirect(
      `/onboarding/github?installation_id=${installation.installation_id}`,
    );
  } else {
    redirect(
      `/settings/migrations/new?installation_id=${installation.installation_id}`,
    );
  }
}
