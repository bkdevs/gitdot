import Image from "next/image";
import { OwnerCenter } from "./ui/owner-center";

const FAKE_REPOS = [
  { name: "gitdot", stars: 847 },
  { name: "s2-sdk-rs", stars: 312 },
  { name: "axum-auth", stars: 204 },
  { name: "pg-migrate", stars: 178 },
  { name: "dotfiles", stars: 94 },
  { name: "advent-of-code", stars: 41 },
  { name: "til", stars: 29 },
  { name: "scratchpad", stars: 4 },
];

export default async function Page({
  params,
}: {
  params: Promise<{ owner: string }>;
}) {
  const { owner } = await params;

  return (
    <div className="grid grid-cols-[1fr_600px_1fr] items-start py-10">
      {/* Left: profile sidebar */}
      <div className="border-r flex flex-col items-end px-4 gap-6">
        <div className="flex flex-col items-end">
          <Image
            src="/paul-penguin.jpeg"
            alt={owner}
            width={32}
            height={32}
            className="rounded-full"
          />
          <p className="font-semibold text-sm">{owner}</p>
          <p className="text-sm text-muted-foreground">paul@gitdot.io</p>
          <p className="text-sm text-muted-foreground">brooklyn, ny</p>
        </div>

        <div className="flex flex-col items-end">
          <p className="font-semibold text-sm mb-1">repos</p>
          <div className="flex flex-col items-end gap-1">
            {FAKE_REPOS.map((repo) => (
              <div key={repo.name} className="flex items-baseline gap-1.5">
                <span className="text-xs">{repo.name}</span>
                <span className="text-xs text-muted-foreground">({repo.stars})</span>
              </div>
            ))}
          </div>
        </div>

      </div>

      {/* Center: interactive content */}
      <OwnerCenter owner={owner} />

      {/* Right: empty */}
      <div />
    </div>
  );
}
