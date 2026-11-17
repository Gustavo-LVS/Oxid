import { defineConfig } from "astro/config";

// Site estático (Cloudflare Pages ou Vercel). Troque `site` pelo domínio final.
export default defineConfig({
  site: "https://oxid.app",
  output: "static",
});
