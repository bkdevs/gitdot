import { getCurrentUser } from "@/dal";
import { SettingsSidebar } from "./ui/settings-sidebar";

export default async function Layout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  await getCurrentUser();

  return (
    <>
      <div className="flex md:hidden h-full w-full p-2 text-sm">
        Mobile support to come.
      </div>

      <div className="hidden md:flex h-full w-full">
        <SettingsSidebar />
        <div className="flex-1 min-w-0 overflow-auto scrollbar-thin">
          {children}
        </div>
      </div>
    </>
  );
}
