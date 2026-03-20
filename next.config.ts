import type { NextConfig } from "next";

const isProd = process.env.NODE_ENV === "production";
const internalHost = process.env.TAURI_DEV_HOST || "localhost";

const nextConfig: NextConfig = {
  output: "export", // Outputs a Single-Page Application (SPA)
  distDir: "dist", // Changes the build output directory to `dist` (matches Tauri config)

  images: {
    unoptimized: true,
  },

  // Configure assetPrefix or else the server won't properly resolve your assets.
  assetPrefix: isProd ? undefined : `http://${internalHost}:3000`,
};

export default nextConfig;
