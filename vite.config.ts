import path from "node:path";
import { reactRouter } from "@react-router/dev/vite";
import tailwindcss from "@tailwindcss/vite";
import { defineConfig } from "vite";

export default defineConfig({
  plugins: [tailwindcss(), reactRouter()],
  resolve: {
    alias: {
      "@": path.resolve(import.meta.dirname, "./src"),
    },
  },
  optimizeDeps: {
    // Tauri's WebKit view can retain the server-rendered loading screen when
    // Vite discovers a dependency after navigation and forces a page reload.
    // Prebundle the browser imports once before serving the app.
    noDiscovery: true,
    include: [
      "@dagrejs/dagre",
      "@radix-ui/react-dialog",
      "@tauri-apps/api/core",
      "@tauri-apps/api/webview",
      "@xyflow/react",
      "clsx",
      "lucide-react",
      "react",
      "react-dom",
      "react-markdown",
      "react-router",
      "remark-gfm",
      "tailwind-merge",
      "zustand",
    ],
  },
  server: {
    watch: { ignored: ["**/src-tauri/**", "**/target/**"] },
  },
});
