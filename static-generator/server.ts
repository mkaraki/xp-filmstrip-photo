import { Hono } from "hono";
import { serveStatic } from "hono/bun";
import { join } from "path";
import { existsSync, readFileSync } from "fs";

const app = new Hono();
const PORT = 3001;
// Ensure we use an absolute path for the public directory
const PUBLIC_DIR = join(import.meta.dir, "dist_static");

console.log(`--- Starting Hono Static Test Server on http://localhost:${PORT} ---`);

// 1. Handle API and Nuxt assets strictly
app.use("/.__api/*", serveStatic({ 
  root: "./dist_static",
  rewriteRequestPath: (path) => path // keep full path
}));

app.use("/_nuxt/*", serveStatic({ 
  root: "./dist_static",
  rewriteRequestPath: (path) => path
}));

// 2. Try serving files directly from the root
app.use("*", async (c, next) => {
  const url = new URL(c.req.url);
  const path = decodeURIComponent(url.pathname);
  
  const filePath = join(PUBLIC_DIR, path);
  // If it's a file that exists, serve it using standard static middleware
  if (path !== "/" && existsSync(filePath) && !path.startsWith("/.__api") && !path.startsWith("/_nuxt")) {
    return serveStatic({ root: "./dist_static" })(c, next);
  }
  await next();
});

// 3. SPA Fallback: Serve index.html for any remaining non-file routes
app.get("*", async (c) => {
  const url = new URL(c.req.url);
  const path = decodeURIComponent(url.pathname);

  // Don't fallback for missing API or Nuxt data
  if (path.startsWith("/.__api") || path.startsWith("/_nuxt")) {
    return c.text("Not Found", 404);
  }

  // Use readFileSync to ensure we send the actual content, not a Blob object description
  try {
    const html = readFileSync(join(PUBLIC_DIR, "index.html"), "utf-8");
    return c.html(html);
  } catch (e) {
    return c.text("index.html not found. Run generate first.", 404);
  }
});

export default {
  port: PORT,
  fetch: app.fetch,
};
