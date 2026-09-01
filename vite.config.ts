import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

export default defineConfig({
  plugins: [react()],
  clearScreen: false,
  server: { port: 1420, strictPort: true, proxy: { "/api": "http://localhost:3000" } },
  envPrefix: ["VITE_", "TAURI_"]
});
