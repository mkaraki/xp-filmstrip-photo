import { beforeEach, describe, expect, it } from "bun:test";
import { emitStorageEvent } from "../setup/browserMocks";
import "../setup/browserMocks";
import { useAuth } from "../../composables/useAuth";
import { useApiFetch } from "../../composables/useApiFetch";

describe("useApiFetch", () => {
  beforeEach(() => {
    const { clearCredentials } = useAuth();
    clearCredentials();
  });

  it("appends .json for internal API paths and sends auth header", async () => {
    const { setCredentials } = useAuth();
    setCredentials("alice", "secret", true);

    let requestedUrl = "";
    let authorization = "";

    (globalThis as any).fetch = async (url: string, options: RequestInit) => {
      requestedUrl = url;
      const headers = options.headers as Headers;
      authorization = headers.get("Authorization") || "";
      return { status: 200, ok: true };
    };

    const { fetchApi } = useApiFetch();
    const response = await fetchApi("/.__api/folder/photos");

    expect(requestedUrl).toBe("/.__api/folder/photos.json");
    expect(authorization).toBe("Basic YWxpY2U6c2VjcmV0");
    expect(response.ok).toBe(true);
  });

  it("does not append .json to non-internal paths", async () => {
    let requestedUrl = "";

    (globalThis as any).fetch = async (url: string) => {
      requestedUrl = url;
      return { status: 200, ok: true };
    };

    const { fetchApi } = useApiFetch();
    await fetchApi("/images/a.png");

    expect(requestedUrl).toBe("/images/a.png");
  });

  it("normalizes trailing slash before appending .json for internal API paths", async () => {
    let requestedUrl = "";

    (globalThis as any).fetch = async (url: string) => {
      requestedUrl = url;
      return { status: 200, ok: true };
    };

    const { fetchApi } = useApiFetch();
    await fetchApi("/.__api/list/foo/");

    expect(requestedUrl).toBe("/.__api/list/foo.json");
  });

  it("opens login dialog on 401 and reloads after storage auth update", async () => {
    let openCalls = 0;
    let reloadCalls = 0;

    (window.open as any) = () => {
      openCalls += 1;
      return { focus: () => {} };
    };
    (window.location.reload as any) = () => {
      reloadCalls += 1;
    };
    (globalThis as any).fetch = async () => ({ status: 401, ok: false });

    const { fetchApi } = useApiFetch();
    await fetchApi("/.__api/version");
    emitStorageEvent("filmstrip_auth", "new-auth-token");

    expect(openCalls).toBe(1);
    expect(reloadCalls).toBe(1);
  });
});
