import NextLink from "next/link";
import type { AnchorHTMLAttributes, ReactNode } from "react";

interface SmartLinkProps
  extends Omit<AnchorHTMLAttributes<HTMLAnchorElement>, "href"> {
  href: string;
  prefetch?: boolean;
  children: ReactNode;
}

/**
 * So the reason we need this hacky wrapper (and the biome rule that tells us to use this hacky wrapper)
 * is that we want to keep file paths as true as possible in the URL, e.g., you can copy paste pwd into the browser and it will work.
 *
 * next.js is defensive about letting dynamic slugs in the URL, so Link will in fact fail if we attempt to give it a href that has [slug] in it.
 * so if we detect that the href has a dynamic segment, we do a plain old <a> instead
 *
 * this isn't ideal, still, because next.js will attempt to hydrate the dynamic slug regardless, meaning there's a bit of client-side flicker induced
 * if the dynamic slug happens to be one of our own (e.g., [owner], [repo], but that shouldn't be an issue if the next.js application the user is hosting does not overlap
 *
 * i suppose we can make our slugs very very long and rare to avoid the issue too.
 */
export default function Link({
  href,
  prefetch = true,
  children,
  ...props
}: SmartLinkProps) {
  const hasDynamicSegment = /\[.*?\]/.test(href);

  if (hasDynamicSegment) {
    // still causes hydration flicker as next.js will attempt to render [owner] as our own path
    return (
      <a href={href} {...props}>
        {children}
      </a>
    );
  }

  return (
    <NextLink href={href} prefetch={prefetch} {...props}>
      {children}
    </NextLink>
  );
}
