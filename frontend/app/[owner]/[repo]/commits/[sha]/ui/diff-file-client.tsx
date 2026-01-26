"use client";

import { Fragment, useState } from "react";
import { DiffHeader } from "./diff-header";

export function DiffFileClient({
  children,
  leftPath,
  rightPath,
  linesAdded,
  linesRemoved,
}: {
  children: React.ReactNode;
  leftPath: string | undefined;
  rightPath: string | undefined;
  linesAdded: number;
  linesRemoved: number;
}) {
  const [diffOpen, setDiffOpen] = useState(true);

  return (
    <Fragment>
      <DiffHeader
        open={diffOpen}
        setOpen={setDiffOpen}
        leftPath={leftPath}
        rightPath={rightPath}
        linesAdded={linesAdded}
        linesRemoved={linesRemoved}
      />
      {diffOpen && children}
    </Fragment>
  );
}
