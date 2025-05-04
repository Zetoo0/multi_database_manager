import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";

type ResponseBody = {
  data: ParsedData;
  error: object | null;
};

type ParsedData = {
  fileType: string;
  parsedTo?: string;
  content: object;
};

const getParsed = async (): Promise<ParsedData> => {
  const resp: ResponseBody = await invoke<ResponseBody>("get_parsed_docx");
  // const resp: ResponseBody =  await invoke<ResponseBody>("get_parsed_md");
  if (resp.error) {
    console.error(resp.error);
  }
  return resp.data;
};

export const useParsed = () => {
  const [parsed, setParsed] = useState({});
  const [parsedTo, setParsedTo] = useState("");

  useEffect(() => {
    getParsed().then(data => {
      setParsed(data.content);
      setParsedTo(data.parsedTo || "");
    });
  }, []);

  return {
    parsed,
    parsedTo
  };
};
