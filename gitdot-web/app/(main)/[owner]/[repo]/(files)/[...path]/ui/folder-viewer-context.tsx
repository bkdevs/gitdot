"use client";

import { createContext, useContext, useState } from "react";

type FolderViewerContextType = {
  hoveredPath: string | null;
  setHoveredPath: (path: string) => void;
  pinnedPath: string | null;
  setPinnedPath: (path: string | null) => void;
  pinFiles: boolean;
  previewPath: string | null;
};

const FolderViewerContext = createContext<FolderViewerContextType | null>(null);

export function useFolderViewerContext() {
  const context = useContext(FolderViewerContext);
  if (!context) {
    throw new Error(
      "useFolderViewerContext must be used within FolderViewerProvider",
    );
  }
  return context;
}

export function FolderViewerProvider({
  pinFiles,
  children,
}: {
  pinFiles: boolean;
  children: React.ReactNode;
}) {
  const [hoveredPath, setHoveredPath] = useState<string | null>(null);
  const [pinnedPath, setPinnedPath] = useState<string | null>(null);

  return (
    <FolderViewerContext.Provider
      value={{
        hoveredPath,
        setHoveredPath,
        pinnedPath,
        setPinnedPath,
        pinFiles,
        previewPath: pinnedPath ?? hoveredPath,
      }}
    >
      {children}
    </FolderViewerContext.Provider>
  );
}
