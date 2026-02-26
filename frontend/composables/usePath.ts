export const normalizePath = (path: string | null | undefined): string => {
  if (!path) return "";
  return path
    .split("/")
    .filter(Boolean)
    .join("/");
};

export const routeSlugPath = (slug: string | string[] | undefined): string => {
  if (Array.isArray(slug)) {
    return normalizePath(slug.join("/"));
  }
  return normalizePath(slug ?? "");
};
