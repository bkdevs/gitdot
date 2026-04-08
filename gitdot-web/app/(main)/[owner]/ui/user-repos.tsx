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

export function UserRepos({ owner }: { owner: string }) {
  return (
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
  );
}
