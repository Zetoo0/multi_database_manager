import { useHotkeys } from "react-hotkeys-hook";

const KeyConfig = {
  openSettings: "ctrl+shift+s",
  closeSettings: "esc",
  createDatabase: "ctrl+n",
};

const baseFormats: { [key: string]: string } = {
  capslock: "⇪",
  shift: "⇧",
  arrowup: "↑",
  arrowright: "→",
  arrowdown: "↓",
  arrowleft: "←",
  enter: "↩",
  backspace: "⌫",
  delete: "⌦",
  escape: "⎋",
  tab: "⇥",
  pageup: "⇞",
  pagedown: "⇟",
  space: "␣",
};

const macFormats: { [key: string]: string } = {
  ctrl: "⌃",
  alt: "⌥",
  option: "⌥",
  meta: "⌘",
  super: "⌘",
  cmd: "⌘",
  command: "⌘",
};

const winFormats: { [key: string]: string } = {
  ctrl: "ctrl",
  option: "alt",
  meta: "❖",
  super: "❖",
  cmd: "❖",
  command: "❖",
};

export const useHotkey = (
  hotkey: keyof typeof KeyConfig,
  callback: () => void
) => {
  useHotkeys(KeyConfig[hotkey], callback);

  const os = "mac";
  const formats: { [key: string]: string } =
    os === "mac" ? macFormats : winFormats;

  return {
    original: KeyConfig[hotkey],
    formatted: KeyConfig[hotkey]
      .replace(/\+/g, " ")
      .split(" ")
      .map(
        (word) =>
          formats[word.toLowerCase()] || baseFormats[word.toLowerCase()] || word
      )
      .map((word) => word.charAt(0).toUpperCase() + word.slice(1))
      .join(" "),
  };
};
