import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";

type SystemInfo = {
  monitor?: MonitorInfo;
  os?: OSInfo;
  [key: string]: any;
};

type MonitorInfo = {
  name: string | null;
  scale_factor: number;
  width: number;
  height: number;
};

type OSInfo = {
  architecture: string;
  bitness: string;
  info: string;
  type: string;
  version: string;
};

type ResponseBody = {
  data: SystemInfo;
  error: object | null;
}

const getSystemInfo = async (): Promise<SystemInfo> => {
  const systemInfo = await invoke<ResponseBody>("get_os_information").then(info=>info.data);

  return systemInfo;
};

export const useSystemInfo = () => {
  const [systemInfo, setSystemInfo] = useState<SystemInfo | null>(null);

  useEffect(() => {
    getSystemInfo().then((info) => {
      setSystemInfo(info);
    });
  }, []);

  return {
    systemInfo,
  };
};
