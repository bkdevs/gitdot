import Image from "next/image";
import Link from "@/ui/link";
import { githubAppInstallUrl } from "@/util";

export function GitHubImport() {
  return (
    <div className="max-w-3xl mx-auto flex gap-4 items-center justify-center h-screen">
      <div className="flex flex-col text-sm w-sm">
        <p className="pb-2">Import.</p>
        <p className="text-primary/60 pb-4">
          You can import your GitHub repositories into gitdot by installing the gitdot
          GitHub App on your account.
        </p>

        <a
          href={githubAppInstallUrl("onboarding")}
          className="flex items-center justify-center gap-2 border border-border py-1.5 hover:bg-gray-50 transition-colors duration-150"
        >
          <Image src="/github-logo.svg" alt="GitHub" width={16} height={16} />
          Install GitHub App
        </a>

        <div className="flex justify-end mt-2">
          <Link href="/home" className="decoration-primary/40">
            Skip.
          </Link>
        </div>
      </div>
    </div>
  );
}
