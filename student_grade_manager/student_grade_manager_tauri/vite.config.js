import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";

// https://vitejs.dev/config/
export default defineConfig(async () => ({
  plugins: [vue()],
  clearScreen: false,
  server: {
    port: 3000, // 改为3000，避免1420端口冲突
    strictPort: true,
    host: '127.0.0.1', // 强制使用IPv4，避免::1权限问题
    hmr: {
      protocol: "ws",
      host: '127.0.0.1',
      port: 3001,
    },
    watch: {
      ignored: ["**/src-tauri/**"],
    },
  },
}));
