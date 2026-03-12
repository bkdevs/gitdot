"use client";

import { createContext, useContext, useEffect, useState } from "react";
import { type Database, openIdb } from "@/db";

interface DatabaseContext {
  db: Database | null;
}

const DatabaseContext = createContext<DatabaseContext | null>(null);

export function DatabaseProvider({ children }: { children: React.ReactNode }) {
  const [db, setDb] = useState<Database | null>(null);

  useEffect(() => {
    openIdb().then(setDb);
  }, []);

  return <DatabaseContext value={{ db }}>{children}</DatabaseContext>;
}

export function useDatabaseContext(): DatabaseContext {
  const context = useContext(DatabaseContext);
  if (!context) {
    throw new Error(
      "useDatabaseContext must be used within a DatabaseProvider",
    );
  }
  return context;
}
