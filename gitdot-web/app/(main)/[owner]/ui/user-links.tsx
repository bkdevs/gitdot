import type { UserResource } from "gitdot-api";

export function UserLinks({ user }: { user: UserResource }) {
  if (!user.website) return null;

  return (
    <div className="flex flex-col items-end">
      <p className="font-semibold text-sm mb-0.5">links</p>
      <a
        href={/^https?:\/\//.test(user.website) ? user.website : `https://${user.website}`}
        target="_blank"
        rel="noopener noreferrer"
        className="text-xs text-muted-foreground underline decoration-transparent hover:decoration-current transition-colors duration-200"
      >
        {user.website.replace(/^https?:\/\//, "")}
      </a>
    </div>
  );
}
