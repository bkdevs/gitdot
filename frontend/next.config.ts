import type { NextConfig } from "next";

const nextConfig: NextConfig = {
  reactCompiler: true,
  async redirects() {
    return [
      {
        source: "/beta",
        destination: "/bkdevs/gitdot",
        permanent: true,
      },
      {
        source: "/signup",
        destination: "/login",
        permanent: false,
      },
    ];
  },
};

export default nextConfig;
