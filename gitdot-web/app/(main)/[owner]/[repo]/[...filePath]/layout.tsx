import { RepoScroll } from "../ui/scroll";
import { FileSidebar } from "./ui/sidebar";

export default function Layout({ children }: { children: React.ReactNode }) {
  return (
    <>
      <FileSidebar />
      <RepoScroll>{children}</RepoScroll>
    </>
  );
}
