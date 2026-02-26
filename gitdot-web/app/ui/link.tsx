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
 * https://nextjs.org/docs/messages/app-dir-dynamic-href
 * https://github.com/vercel/next.js/blob/b9edb9175e15b433122afb114cbec6a2951d7d02/packages/next/src/client/app-dir/link.tsx#L505-L515
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
  // whenever we see any path that looks like gitdot.io/org/org, we de-duplicate it
  // this relies on the fact that the middleware will rewrite org to org/org, so we retain pretty paths
  const canonicalHref = href.replace(/^(\/?)([^\/]+)\/\2(\/|$)/, "$1$2$3");
  const hasDynamicSegment = /\[.*?\]/.test(canonicalHref);

  if (hasDynamicSegment) {
    // still causes hydration flicker as next.js will attempt to render [owner] as our own path
    return (
      <a href={canonicalHref} {...props}>
        {children}
      </a>
    );
  }

  return (
    <NextLink href={canonicalHref} prefetch={prefetch} {...props}>
      {children}
    </NextLink>
  );
}
