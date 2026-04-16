"use client";

import type { DiffData } from "@/actions";
import type { RepositoryDiffFileResource } from "gitdot-api";
import { useState } from "react";
import { ReviewDiffFileBody } from "./review-diff-file-body";
import { ReviewDiffFileDialog } from "./review-diff-file-dialog";
import { ReviewDiffFileHeader } from "./review-diff-file-header";

export function ReviewDiffFile({
	diff,
	data,
}: {
	diff: RepositoryDiffFileResource;
	data: DiffData;
}) {
	const [open, setOpen] = useState(false);

	return (
		<>
			<div
				data-diff-file
				className="rounded-sm border border-border overflow-hidden"
			>
				<ReviewDiffFileHeader diff={diff} onClick={() => setOpen(true)} />
				<ReviewDiffFileBody data={data} />
			</div>
			<ReviewDiffFileDialog
				diff={diff}
				data={data}
				open={open}
				setOpen={setOpen}
			/>
		</>
	);
}
