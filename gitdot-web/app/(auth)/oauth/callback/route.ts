import { redirect } from "next/navigation";
import type { NextRequest } from "next/server";
import { createInstallation } from "@/dal";
import { exchangeGitHubCode } from "@/lib/auth";

export async function GET(request: NextRequest) {
  const { searchParams } = new URL(request.url);
  const code = searchParams.get("code");
  const state = searchParams.get("state");
  const installationId = searchParams.get("installation_id");

  if (installationId && code && state) {
    const result = await createInstallation(
      Number(installationId),
      code,
      state,
    );
    if (!result) {
      redirect("/home");
      return;
    }

    if (result.action === "onboarding") {
      redirect(
        `/onboarding/github?installation_id=${result.installation.installation_id}`,
      );
    } else {
      redirect("/home");
    }
    return;
  }

  if (!code || !state) {
    redirect("/login");
    return;
  }

  const result = await exchangeGitHubCode(code, state);
  if (!result) {
    redirect("/login");
    return;
  }

  redirect(`/${result.username}`);
}
