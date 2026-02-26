import { describe, expect, it } from "bun:test";
import { normalizePath, routeSlugPath } from "../../composables/usePath";

describe("usePath", () => {
  describe("normalizePath", () => {
    it("returns empty string for empty-ish input", () => {
      expect(normalizePath("")).toBe("");
      expect(normalizePath(undefined)).toBe("");
      expect(normalizePath(null)).toBe("");
    });

    it("normalizes leading and trailing slashes", () => {
      expect(normalizePath("/albums/trip/")).toBe("albums/trip");
    });

    it("normalizes repeated slashes", () => {
      expect(normalizePath("albums//trip///2026")).toBe("albums/trip/2026");
    });
  });

  describe("routeSlugPath", () => {
    it("handles string slug", () => {
      expect(routeSlugPath("albums/trip/")).toBe("albums/trip");
    });

    it("handles array slug", () => {
      expect(routeSlugPath(["albums", "trip", ""])).toBe("albums/trip");
    });

    it("handles undefined slug", () => {
      expect(routeSlugPath(undefined)).toBe("");
    });
  });
});
