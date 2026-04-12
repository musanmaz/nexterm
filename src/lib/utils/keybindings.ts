type KeyHandler = () => void;

interface Keybinding {
  key: string;
  ctrl?: boolean;
  shift?: boolean;
  alt?: boolean;
  handler: KeyHandler;
  description: string;
}

class KeybindingsManager {
  private bindings: Keybinding[] = [];
  private active = true;

  register(binding: Keybinding) {
    this.bindings.push(binding);
  }

  unregister(key: string) {
    this.bindings = this.bindings.filter((b) => b.key !== key);
  }

  init() {
    window.addEventListener('keydown', (e) => {
      if (!this.active) return;
      for (const b of this.bindings) {
        if (
          e.key.toLowerCase() === b.key.toLowerCase() &&
          !!e.ctrlKey === !!b.ctrl &&
          !!e.shiftKey === !!b.shift &&
          !!e.altKey === !!b.alt
        ) {
          e.preventDefault();
          b.handler();
          return;
        }
      }
    });
  }

  setActive(active: boolean) {
    this.active = active;
  }

  getAll(): Keybinding[] {
    return [...this.bindings];
  }
}

export const keybindings = new KeybindingsManager();
