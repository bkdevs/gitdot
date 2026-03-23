import { OverlayScroll } from "../../../../ui/scroll";
import { FileSidebar } from "./ui/sidebar";

export default function Layout({ children }: { children: React.ReactNode }) {
  return (
    <>
      <FileSidebar />
      <OverlayScroll>{children}</OverlayScroll>
    </>
  );
}
