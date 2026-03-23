import { OverlayScroll } from "../../../../../ui/scroll";
import { CommitSidebar } from "./ui/sidebar";

export default function Layout({ children }: { children: React.ReactNode }) {
  return (
    <>
      <CommitSidebar />
      <OverlayScroll>{children}</OverlayScroll>
    </>
  );
}
