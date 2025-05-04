import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";

export const useMemoryUsage = () => {
  const [memoryUsage, setMemoryUsage] = useState("");

  const updateMemoryUsage = async () => {
    const bytes: number = await invoke("get_vram_usage");

    if (bytes === 0) {
      setMemoryUsage("-- MB");
    } else {
      setMemoryUsage(`${(bytes / 8 / 1024 / 1024 / 1024).toFixed(0)} MB`);
    }
  };

  useEffect(() => {
    updateMemoryUsage(); // Call updateMemoryUsage initially

    const intervalId = setInterval(updateMemoryUsage, 1000); // Update every 1 second

    return () => {
      clearInterval(intervalId); // Clear the interval when the component is unmounted
    };
  }, []);

  return {
    memoryUsage,
  };
};
