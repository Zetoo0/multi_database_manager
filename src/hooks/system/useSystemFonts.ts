import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";

const getSystemFonts = async (): Promise<string[]> => {
  return await invoke("get_system_fonts");
};

export const useSystemFonts = () => {
  const [systemFonts, setSystemFonts] = useState<string[]>([]);

  useEffect(() => {
    async function fetchFonts() {
      try {
        const fonts = await getSystemFonts();
        setSystemFonts(fonts);
      } catch (error) {
        console.error("Error fetching fonts:", error);
      }
    }
    fetchFonts();
  }, []);

  return {
    systemFonts,
  };
};
