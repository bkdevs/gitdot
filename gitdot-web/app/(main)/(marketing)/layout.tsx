import { getCurrentUser } from "gitdot-client";
import { LayoutClient } from "./layout.client";

export default async function MarketingLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  const user = await getCurrentUser(false);

  return <LayoutClient user={user}>{children}</LayoutClient>;
}
