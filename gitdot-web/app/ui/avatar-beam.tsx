"use client";

import * as React from "react";

const SIZE = 36;

function hashCode(name: string): number {
  let hash = 0;
  for (let i = 0; i < name.length; i++) {
    const character = name.charCodeAt(i);
    hash = (hash << 5) - hash + character;
    hash = hash & hash;
  }
  return Math.abs(hash);
}

function getUnit(n: number, range: number, index?: number): number {
  const value = n % range;
  if (index && (n % (index * range)) / index < range) {
    return -value;
  }
  return value;
}

function getBoolean(n: number, n2: number): boolean {
  return !!(n % n2);
}

function getRandomColor(n: number, colors: string[], range: number): string {
  return colors[n % range];
}

function getContrast(hex: string): string {
  const r = Number.parseInt(hex.slice(1, 3), 16);
  const g = Number.parseInt(hex.slice(3, 5), 16);
  const b = Number.parseInt(hex.slice(5, 7), 16);
  const yiq = (r * 299 + g * 587 + b * 114) / 1000;
  return yiq >= 128 ? "#000000" : "#ffffff";
}

const COLORS = ["#92A1C6", "#146A7C", "#F0AB3D", "#C271B4", "#C20D90"];

function generateData(name: string, colors: string[]) {
  const numFromName = hashCode(name);
  const range = colors.length;
  const wrapperColor = getRandomColor(numFromName, colors, range);
  const preTranslateX = getUnit(numFromName, 10, 1);
  const wrapperTranslateX =
    preTranslateX < 5 ? preTranslateX + SIZE / 9 : preTranslateX;
  const preTranslateY = getUnit(numFromName, 10, 2);
  const wrapperTranslateY =
    preTranslateY < 5 ? preTranslateY + SIZE / 9 : preTranslateY;

  return {
    wrapperColor,
    faceColor: getContrast(wrapperColor),
    backgroundColor: getRandomColor(numFromName + 13, colors, range),
    wrapperTranslateX,
    wrapperTranslateY,
    wrapperRotate: getUnit(numFromName, 360),
    wrapperScale: 1 + getUnit(numFromName, SIZE / 12) / 10,
    isMouthOpen: getBoolean(numFromName, 2),
    isCircle: getBoolean(numFromName, 1),
    eyeSpread: getUnit(numFromName, 5),
    mouthSpread: getUnit(numFromName, 3),
    faceRotate: getUnit(numFromName, 10, 3),
    faceTranslateX:
      wrapperTranslateX > SIZE / 6
        ? wrapperTranslateX / 2
        : getUnit(numFromName, 8, 1),
    faceTranslateY:
      wrapperTranslateY > SIZE / 6
        ? wrapperTranslateY / 2
        : getUnit(numFromName, 7, 2),
  };
}

export function AvatarBeam({ name, size }: { name: string; size: number }) {
  const data = generateData(name, COLORS);
  const maskID = React.useId();

  return (
    <svg
      viewBox={`0 0 ${SIZE} ${SIZE}`}
      fill="none"
      role="img"
      aria-label={name}
      xmlns="http://www.w3.org/2000/svg"
      width={size}
      height={size}
    >
      <mask
        id={maskID}
        maskUnits="userSpaceOnUse"
        x={0}
        y={0}
        width={SIZE}
        height={SIZE}
      >
        <rect width={SIZE} height={SIZE} rx={SIZE * 2} fill="#FFFFFF" />
      </mask>
      <g mask={`url(#${maskID})`}>
        <rect width={SIZE} height={SIZE} fill={data.backgroundColor} />
        <rect
          x="0"
          y="0"
          width={SIZE}
          height={SIZE}
          transform={
            "translate(" +
            data.wrapperTranslateX +
            " " +
            data.wrapperTranslateY +
            ") rotate(" +
            data.wrapperRotate +
            " " +
            SIZE / 2 +
            " " +
            SIZE / 2 +
            ") scale(" +
            data.wrapperScale +
            ")"
          }
          fill={data.wrapperColor}
          rx={data.isCircle ? SIZE : SIZE / 6}
        />
        <g
          transform={
            "translate(" +
            data.faceTranslateX +
            " " +
            data.faceTranslateY +
            ") rotate(" +
            data.faceRotate +
            " " +
            SIZE / 2 +
            " " +
            SIZE / 2 +
            ")"
          }
        >
          {data.isMouthOpen ? (
            <path
              d={`M15 ${19 + data.mouthSpread}c2 1 4 1 6 0`}
              stroke={data.faceColor}
              fill="none"
              strokeLinecap="round"
            />
          ) : (
            <path
              d={`M13,${19 + data.mouthSpread} a1,0.75 0 0,0 10,0`}
              fill={data.faceColor}
            />
          )}
          <rect
            x={14 - data.eyeSpread}
            y={14}
            width={1.5}
            height={2}
            rx={1}
            stroke="none"
            fill={data.faceColor}
          />
          <rect
            x={20 + data.eyeSpread}
            y={14}
            width={1.5}
            height={2}
            rx={1}
            stroke="none"
            fill={data.faceColor}
          />
        </g>
      </g>
    </svg>
  );
}
