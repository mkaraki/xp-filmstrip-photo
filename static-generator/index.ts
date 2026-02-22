import { join, dirname } from "node:path";
import { mkdir, writeFile, copyFile, cp, utimes, stat } from "node:fs/promises";
import fs from "node:fs";


declare const process: any;

const PHOTO_ROOT = "../fixtures";
const BACKEND_URL = "http://localhost:8080";
const OUTPUT_DIR = "./dist_static";
const FRONTEND_OUTPUT = "../frontend/.output/public";

// Load version from package.json
const pkg = JSON.parse(fs.readFileSync("./package.json", "utf-8"));
const PINNER_VERSION = pkg.version;

// Capture current time for pinner build time
const now = new Date();
const pad = (n: number) => n.toString().padStart(2, '0');
const pinnerBuildTime = `${now.getFullYear()}.${pad(now.getMonth() + 1)}${pad(now.getDate())}.${pad(now.getHours())}${pad(now.getMinutes())}${pad(now.getSeconds())}`;

const processedDirs = new Set<string>();

/**
 * Checks if a local file is fresh compared to a target timestamp.
 */
async function isFresh(targetPath: string, serverLastModified: string | null): Promise<boolean> {
  if (!serverLastModified || !fs.existsSync(targetPath)) return false;
  try {
    const localStat = await stat(targetPath);
    const serverTime = new Date(serverLastModified).getTime();
    const localTime = localStat.mtime.getTime();
    return Math.abs(serverTime - localTime) < 2000;
  } catch {
    return false;
  }
}

/**
 * Handles writing the response content to disk, distinguishing between JSON and binary.
 */
async function writeResponse(targetPath: string, res: Response, relPath: string, transform?: (data: any) => any) {
  await mkdir(dirname(targetPath), { recursive: true });

  const contentType = res.headers.get("content-type") || "";
  const isJson = contentType.includes("application/json") || contentType.includes("text/plain") || relPath.endsWith(".json");

  if (!isJson) {
    const buffer = Buffer.from(await res.arrayBuffer());
    await writeFile(targetPath, buffer);
    const lastMod = res.headers.get("last-modified");
    if (lastMod) {
      await utimes(targetPath, new Date(), new Date(lastMod));
    }
    return null;
  }

  let data: any = await res.text();
  if (transform) data = transform(data);
  await writeFile(targetPath, data);

  try {
    return JSON.parse(data);
  } catch {
    return data;
  }
}

/**
 * Fetches a file from the backend and saves it locally if it's new or changed.
 */
async function saveFile(relPath: string, url: string, transform?: (data: any) => any) {
  const targetPath = join(OUTPUT_DIR, relPath);
  const isBinary = !url.includes(".json") && !url.includes("version");

  if (isBinary && fs.existsSync(targetPath)) {
    const headRes = await fetch(url, { method: "HEAD" });
    if (await isFresh(targetPath, headRes.headers.get("last-modified"))) {
      return null;
    }
  }

  const res = await fetch(url);
  if (!res.ok) {
    console.error(`Failed to fetch ${url}: ${res.status}`);
    return null;
  }

  return await writeResponse(targetPath, res, relPath, transform);
}

/**
 * Synchronizes a local fixture file to the output directory.
 */
async function syncLocalFile(item: { path: string }) {
  const targetFilePath = join(OUTPUT_DIR, item.path);
  const sourceFilePath = join(PHOTO_ROOT, item.path);

  if (fs.existsSync(targetFilePath)) {
    const sStat = await stat(sourceFilePath);
    try {
      const tStat = await stat(targetFilePath);
      if (Math.abs(sStat.mtime.getTime() - tStat.mtime.getTime()) < 2000) {
        return;
      }
    } catch {
      // If stat fails, we proceed to copy
    }
  }

  await mkdir(dirname(targetFilePath), { recursive: true });
  await copyFile(sourceFilePath, targetFilePath);
  const sStat = await stat(sourceFilePath);
  await utimes(targetFilePath, new Date(), sStat.mtime);
}

/**
 * Recursively scrapes folders from the API.
 */
async function scrapeFolder(relPath: string) {
  if (processedDirs.has(relPath)) return;
  processedDirs.add(relPath);

  console.log(`Processing folder: /${relPath || "Root"}`);
  const apiSuffix = relPath ? `/${relPath}.json` : ".json";

  await saveFile(`/.__api/dirs${apiSuffix}`, `${BACKEND_URL}/.__api/dirs${apiSuffix}`);
  const items = await saveFile(`/.__api/list${apiSuffix}`, `${BACKEND_URL}/.__api/list${apiSuffix}`);

  if (!items || !Array.isArray(items)) return;

  for (const item of items) {
    if (item.is_dir) {
      await scrapeFolder(item.path);
    } else {
      await syncLocalFile(item);
      if (item.mime?.startsWith("image/")) {
        await saveFile(`/.__api/metadata/${item.path}.json`, `${BACKEND_URL}/.__api/metadata/${item.path}.json`);
        await saveFile(`/.__api/thumb/${item.path}`, `${BACKEND_URL}/.__api/thumb/${item.path}`);
      }
    }
  }
}

console.log("--- Starting Incremental Static Site Generation ---");

if (!fs.existsSync(OUTPUT_DIR)) {
  await mkdir(OUTPUT_DIR, { recursive: true });
}

console.log("Copying Nuxt frontend files...");
if (!fs.existsSync(FRONTEND_OUTPUT)) {
  console.error("Nuxt build not found! Run 'cd frontend; bun run generate' first.");
  process.exit(1);
}
await cp(FRONTEND_OUTPUT, OUTPUT_DIR, { recursive: true });

console.log("Fetching version info...");
await saveFile("/.__api/version.json", `${BACKEND_URL}/.__api/version.json`, (jsonText) => {
  const info = JSON.parse(jsonText);
  return JSON.stringify({
    ...info,
    pinner_version: PINNER_VERSION,
    pinner_build_time: pinnerBuildTime
  });
});

try {
  await scrapeFolder("");
  console.log("\n--- Generation Complete ---");
  console.log(`Output available in: static-generator/${OUTPUT_DIR}`);
} catch (e) {
  console.error("Scraping failed. Is the backend running at " + BACKEND_URL + "?");
  console.error(e);
}

