type Listener = (event: any) => void;

class MemoryStorage {
  private store = new Map<string, string>();

  getItem(key: string): string | null {
    return this.store.has(key) ? this.store.get(key)! : null;
  }

  setItem(key: string, value: string): void {
    this.store.set(key, String(value));
  }

  removeItem(key: string): void {
    this.store.delete(key);
  }

  clear(): void {
    this.store.clear();
  }
}

const storage = new MemoryStorage();
const storageListeners = new Set<Listener>();

const windowMock = {
  screen: { width: 1920, height: 1080 },
  location: {
    reload: () => {}
  },
  open: () => ({ focus: () => {} }),
  addEventListener: (type: string, cb: Listener) => {
    if (type === "storage") {
      storageListeners.add(cb);
    }
  },
  removeEventListener: (type: string, cb: Listener) => {
    if (type === "storage") {
      storageListeners.delete(cb);
    }
  }
};

const btoaPolyfill = (value: string) => Buffer.from(value, "utf8").toString("base64");

Object.assign(globalThis, {
  window: windowMock,
  localStorage: storage,
  btoa: btoaPolyfill
});

export const emitStorageEvent = (key: string, newValue: string | null) => {
  const event = { key, newValue };
  storageListeners.forEach((listener) => listener(event));
};
