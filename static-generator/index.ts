import { join, dirname } from "path";
import { mkdir, writeFile, copyFile, cp, utimes, stat } from "fs/promises";
import fs from "fs";

const PHOTO_ROOT = "../fixtures";
const BACKEND_URL = "http://localhost:8080";
const OUTPUT_DIR = "./dist_static";
const FRONTEND_OUTPUT = "../frontend/.output/public";

// Load version from package.json
const pkg = JSON.parse(fs.readFileSync("./package.json", "utf-8"));
const PINNER_VERSION = pkg.version;

async function scrape() {
  console.log("--- Starting Incremental Static Site Generation ---");

  // Create output dir if not exists (don't delete)
  if (!fs.existsSync(OUTPUT_DIR)) {
    await mkdir(OUTPUT_DIR, { recursive: true });
  }

  console.log("Copying Nuxt frontend files...");
  if (!fs.existsSync(FRONTEND_OUTPUT)) {
    console.error("Nuxt build not found! Run 'cd frontend; bun run generate' first.");
    process.exit(1);
  }
  // Use cp with recursive but we'll assume Nuxt files might change often
  await cp(FRONTEND_OUTPUT, OUTPUT_DIR, { recursive: true });

  const saveFile = async (relPath: string, url: string, transform?: (data: any) => any) => {
    const targetPath = join(OUTPUT_DIR, relPath);
    
    // For images/thumbs, check modified date
    const isBinary = !url.includes(".json") && !url.includes("version");
    
    if (isBinary && fs.existsSync(targetPath)) {
      const headRes = await fetch(url, { method: "HEAD" });
      const serverLastModified = headRes.headers.get("last-modified");
      
      if (serverLastModified) {
        const localStat = await stat(targetPath);
        const serverTime = new Date(serverLastModified).getTime();
        const localTime = localStat.mtime.getTime();
        
        // If times match (approx), skip
        if (Math.abs(serverTime - localTime) < 2000) {
          return null; 
        }
      }
    }

    console.log(`Fetching file: ${url}`);
    const res = await fetch(url);
    if (!res.ok) {
      console.error(`Failed to fetch ${url}: ${res.status}`);
      return null;
    }
    
    await mkdir(dirname(targetPath), { recursive: true });

    const contentType = res.headers.get("content-type") || "";
    if (contentType.includes("application/json") || contentType.includes("text/plain") || relPath.endsWith(".json")) {
      let data: any = await res.text();
      if (transform) data = transform(data);
      await writeFile(targetPath, data);
      try { return JSON.parse(data); } catch { return data; }
    } else {
      const buffer = Buffer.from(await res.arrayBuffer());
      await writeFile(targetPath, buffer);
      
      // Sync modification time if provided by server
      const lastMod = res.headers.get("last-modified");
      if (lastMod) {
        const mtime = new Date(lastMod);
        await utimes(targetPath, new Date(), mtime);
      }
      return null;
    }
  };

  console.log("Fetching version info...");
  await saveFile("/.__api/version.json", `${BACKEND_URL}/.__api/version.json`, (text) => {
    return `${text.trim()}+pinner${PINNER_VERSION}`;
  });

  const processedDirs = new Set();

  const scrapeFolder = async (relPath: string) => {
    if (processedDirs.has(relPath)) return;
    processedDirs.add(relPath);

    console.log(`Processing folder: /${relPath || "Root"}`);
    
    const apiSuffix = relPath ? `/${relPath}.json` : ".json";
    
    await saveFile(`/.__api/dirs${apiSuffix}`, `${BACKEND_URL}/.__api/dirs${apiSuffix}`);
    const items = await saveFile(`/.__api/list${apiSuffix}`, `${BACKEND_URL}/.__api/list${apiSuffix}`);

    if (items && Array.isArray(items)) {
      for (const item of items) {
        if (item.is_dir) {
          await scrapeFolder(item.path);
        } else {
          // Check original file incremental sync
          const targetFilePath = join(OUTPUT_DIR, item.path);
          const sourceFilePath = join(PHOTO_ROOT, item.path);
          
          let skipCopy = false;
          if (fs.existsSync(targetFilePath)) {
            const sStat = await stat(sourceFilePath);
            const tStat = await stat(targetFilePath);
            if (Math.abs(sStat.mtime.getTime() - tStat.mtime.getTime()) < 2000) {
              skipCopy = true;
            }
          }

          if (!skipCopy) {
            try {
              await mkdir(dirname(targetFilePath), { recursive: true });
              await copyFile(sourceFilePath, targetFilePath);
              // Sync mtime
              const sStat = await stat(sourceFilePath);
              await utimes(targetFilePath, new Date(), sStat.mtime);
            } catch (e) {
              console.warn(`  Warning: Could not copy original file ${item.path}`);
            }
          }

          if (item.mime?.startsWith("image/")) {
            await saveFile(`/.__api/metadata/${item.path}.json`, `${BACKEND_URL}/.__api/metadata/${item.path}.json`);
            await saveFile(`/.__api/thumb/${item.path}`, `${BACKEND_URL}/.__api/thumb/${item.path}`);
          }
        }
      }
    }
  };

  try {
    await scrapeFolder("");
    console.log("\n--- Generation Complete ---");
    console.log(`Output available in: static-generator/${OUTPUT_DIR}`);
  } catch (e) {
    console.error("Scraping failed. Is the backend running at " + BACKEND_URL + "?");
    console.error(e);
  }
}

scrape();
