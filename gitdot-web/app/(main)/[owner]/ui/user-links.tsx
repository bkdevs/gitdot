import type { UserResource } from "gitdot-api";

export function UserLinks({ user }: { user: UserResource }) {
  if (!user.links?.length) return null;

  return (
    <div className="flex flex-col items-end">
      <p className="font-semibold text-sm mb-0.5">links</p>
      {user.links.map((link, i) => (
        <a
          // biome-ignore lint/suspicious/noArrayIndexKey: stable list
          key={i}
          href={/^https?:\/\//.test(link) ? link : `https://${link}`}
          target="_blank"
          rel="noopener noreferrer"
          className="text-xs underline decoration-transparent hover:decoration-current transition-colors duration-200"
        >
          {link.replace(/^https?:\/\//, "")}
        </a>
      ))}
    </div>
  );
}
