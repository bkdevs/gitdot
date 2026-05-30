import type { UserResource } from "gitdot-api";
import Image from "next/image";
import { cn } from "@/util";

type Brand = { src: string; alt: string; pattern: RegExp; className?: string };

const BRANDS: Brand[] = [
  {
    src: "/linkedin-logo.svg",
    alt: "LinkedIn",
    pattern: /^linkedin\.com\/in\/([^/?#]+)/i,
  },
  {
    src: "/x-logo.svg",
    alt: "X",
    pattern: /^(?:twitter|x)\.com\/([^/?#]+)/i,
    className: "dark:invert",
  },
  {
    src: "/github-logo.svg",
    alt: "GitHub",
    pattern: /^github\.com\/([^/?#]+)/i,
    className: "dark:invert",
  },
];

function matchBrand(link: string): { brand: Brand; handle: string } | null {
  const clean = link
    .replace(/^https?:\/\//, "")
    .replace(/^www\./, "")
    .replace(/\/+$/, "");

  for (const brand of BRANDS) {
    const m = clean.match(brand.pattern);
    if (m) return { brand, handle: m[1] };
  }
  return null;
}

export function UserLinks({ user }: { user: UserResource }) {
  if (!user.links?.length) return null;

  return (
    <div className="flex flex-col items-end">
      <p className="font-semibold dark:font-normal text-sm mb-0.5">links</p>
      {user.links.map((link, i) => {
        const matched = matchBrand(link);
        return (
          <a
            key={i}
            href={/^https?:\/\//.test(link) ? link : `https://${link}`}
            target="_blank"
            rel="noopener noreferrer"
            className="flex items-center gap-1 text-xs underline decoration-transparent hover:decoration-current transition-colors duration-200"
          >
            {matched ? (
              <>
                <Image
                  src={matched.brand.src}
                  alt={matched.brand.alt}
                  width={14}
                  height={14}
                  className={cn("shrink-0", matched.brand.className)}
                />
                {matched.handle}
              </>
            ) : (
              link.replace(/^https?:\/\//, "")
            )}
          </a>
        );
      })}
    </div>
  );
}
