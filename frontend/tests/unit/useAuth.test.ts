import { beforeEach, describe, expect, it } from "bun:test";
import "../setup/browserMocks";
import { useAuth } from "../../composables/useAuth";

describe("useAuth", () => {
  beforeEach(() => {
    const { clearCredentials } = useAuth();
    clearCredentials();
  });

  it("stores credentials in localStorage when remember is enabled", () => {
    const { setCredentials, getCredentials } = useAuth();

    setCredentials("alice", "secret", true);

    expect(localStorage.getItem("filmstrip_auth")).toBe("YWxpY2U6c2VjcmV0");
    expect(getCredentials()).toBe("YWxpY2U6c2VjcmV0");
  });

  it("keeps credentials in-memory when remember is disabled", () => {
    const { setCredentials, getCredentials } = useAuth();

    setCredentials("bob", "hidden", false);

    expect(localStorage.getItem("filmstrip_auth")).toBeNull();
    expect(getCredentials()).toBe("Ym9iOmhpZGRlbg==");
  });

  it("clears both local and session credentials", () => {
    const { setCredentials, clearCredentials, getCredentials } = useAuth();
    setCredentials("user", "pass", true);

    clearCredentials();

    expect(localStorage.getItem("filmstrip_auth")).toBeNull();
    expect(getCredentials()).toBeNull();
  });

  it("opens centered login dialog and focuses the popup", () => {
    let openedUrl = "";
    let openedName = "";
    let openedFeatures = "";
    let focused = false;

    (window.open as any) = (url: string, name: string, features: string) => {
      openedUrl = url;
      openedName = name;
      openedFeatures = features;
      return {
        focus: () => {
          focused = true;
        }
      };
    };

    const { openLoginDialog } = useAuth();
    openLoginDialog();

    expect(openedUrl).toBe("/.__ui/login");
    expect(openedName).toBe("ConnectTo");
    expect(openedFeatures.includes("width=420")).toBe(true);
    expect(openedFeatures.includes("height=280")).toBe(true);
    expect(focused).toBe(true);
  });
});
