import { RepoScroll } from "../../ui/scroll";
import { CommitSidebar } from "./ui/sidebar";

export default function Layout({ children }: { children: React.ReactNode }) {
  return (
    <>
      <CommitSidebar />
      <RepoScroll>{children}</RepoScroll>
    </>
  );
}
