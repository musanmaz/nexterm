import type { TerminalTab } from '$lib/types';

let tabs = $state<TerminalTab[]>([]);
let activeTabId = $state<string>('');

export function getTerminalStore() {
  function addTab(tab: TerminalTab) {
    tabs = [...tabs, tab];
    activeTabId = tab.id;
  }

  function removeTab(id: string) {
    tabs = tabs.filter((t) => t.id !== id);
    if (activeTabId === id && tabs.length > 0) {
      activeTabId = tabs[tabs.length - 1].id;
    }
  }

  function setActive(id: string) {
    activeTabId = id;
    tabs = tabs.map((t) => ({ ...t, isActive: t.id === id }));
  }

  function updateTitle(id: string, title: string) {
    tabs = tabs.map((t) => (t.id === id ? { ...t, title } : t));
  }

  return {
    get tabs() { return tabs; },
    get activeTabId() { return activeTabId; },
    addTab,
    removeTab,
    setActive,
    updateTitle,
  };
}
