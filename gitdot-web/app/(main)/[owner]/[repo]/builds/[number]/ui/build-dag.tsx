import type { TaskResource } from "gitdot-api";

const NODE_W = 128;
const NODE_H = 40;
const SIBLING_GAP = 12;
const LEVEL_GAP = 48;
const PAD = 12;

function computeLayout(tasks: TaskResource[]) {
  const idToTask = new Map(tasks.map((t) => [t.id, t]));
  const levelMap = new Map<string, number>();

  function getLevel(task: TaskResource): number {
    if (levelMap.has(task.id)) return levelMap.get(task.id)!;
    if (task.waits_for.length === 0) {
      levelMap.set(task.id, 0);
      return 0;
    }
    const level = Math.max(
      ...task.waits_for.map((depId) => {
        const dep = idToTask.get(depId);
        return dep ? getLevel(dep) + 1 : 0;
      }),
    );
    levelMap.set(task.id, level);
    return level;
  }

  for (const task of tasks) getLevel(task);

  const numLevels = Math.max(...levelMap.values()) + 1;
  const levels: TaskResource[][] = Array.from({ length: numLevels }, () => []);
  for (const task of tasks) {
    levels[levelMap.get(task.id)!].push(task);
  }

  const posMap = new Map<string, { x: number; y: number }>();
  for (let r = 0; r < levels.length; r++) {
    for (let c = 0; c < levels[r].length; c++) {
      posMap.set(levels[r][c].id, {
        x: PAD + c * (NODE_W + SIBLING_GAP),
        y: PAD + r * (NODE_H + LEVEL_GAP),
      });
    }
  }

  const maxSiblings = Math.max(...levels.map((l) => l.length));
  const totalW =
    PAD * 2 + maxSiblings * NODE_W + Math.max(0, maxSiblings - 1) * SIBLING_GAP;
  const totalH =
    PAD * 2 + numLevels * NODE_H + Math.max(0, numLevels - 1) * LEVEL_GAP;

  return { posMap, totalW, totalH };
}

const STATUS_FILL: Record<string, string> = {
  running: "#dbeafe",
  success: "#dcfce7",
  failure: "#fee2e2",
};

const STATUS_STROKE: Record<string, string> = {
  running: "#3b82f6",
  success: "#22c55e",
  failure: "#ef4444",
};

export function BuildDag({ tasks }: { tasks: TaskResource[] }) {
  if (tasks.length === 0) return null;

  const { posMap, totalW, totalH } = computeLayout(tasks);

  const edges: { from: string; to: string }[] = [];
  for (const task of tasks) {
    for (const depId of task.waits_for) {
      if (posMap.has(depId)) {
        edges.push({ from: depId, to: task.id });
      }
    }
  }

  return (
    <svg width={totalW} height={totalH} className="overflow-visible">
      <defs>
        <marker
          id="arrow"
          markerWidth="6"
          markerHeight="6"
          refX="5"
          refY="3"
          orient="auto"
        >
          <path d="M 0 0 L 6 3 L 0 6 Z" fill="#d1d5db" />
        </marker>
      </defs>

      {edges.map(({ from, to }) => {
        const src = posMap.get(from)!;
        const dst = posMap.get(to)!;
        const x1 = src.x + NODE_W / 2;
        const y1 = src.y + NODE_H;
        const x2 = dst.x + NODE_W / 2;
        const y2 = dst.y;
        const my = (y1 + y2) / 2;
        return (
          <path
            key={`${from}-${to}`}
            d={`M ${x1} ${y1} C ${x1} ${my} ${x2} ${my} ${x2} ${y2}`}
            fill="none"
            stroke="#d1d5db"
            strokeWidth={1.5}
            markerEnd="url(#arrow)"
          />
        );
      })}

      {tasks.map((task) => {
        const pos = posMap.get(task.id)!;
        const fill = STATUS_FILL[task.status] ?? "#f3f4f6";
        const stroke = STATUS_STROKE[task.status] ?? "#9ca3af";
        const label =
          task.name.length > 16 ? `${task.name.slice(0, 14)}…` : task.name;
        return (
          <g key={task.id} transform={`translate(${pos.x}, ${pos.y})`}>
            <rect
              width={NODE_W}
              height={NODE_H}
              rx={6}
              fill={fill}
              stroke={stroke}
              strokeWidth={1.5}
            />
            <text
              x={NODE_W / 2}
              y={NODE_H / 2}
              textAnchor="middle"
              dominantBaseline="middle"
              fontSize={12}
              fill="#111827"
              fontFamily="inherit"
            >
              {label}
            </text>
          </g>
        );
      })}
    </svg>
  );
}
