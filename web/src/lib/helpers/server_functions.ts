import { toast } from "@zerodevx/svelte-toast";
import { example_data } from "$lib/examples/example-data";

import type * as Monaco from "monaco-editor/esm/vs/editor/editor.api";

async function try_fetch(url: string, body: string) {
 try {
  const res = await fetch(url, {
   method: "POST",
   body,
  });

  if (res.status !== 200) {
   toast.push("Unsuccessfull request: HTTP " + res.status, { target: "top" });
   return new Error("Unsuccessfull request: HTTP " + res.status);
  }
  const response = await res.json();
  return response;
 } catch (error) {
  toast.push("Could not connect to server", { target: "top" });
  return error;
 }
}

export async function validate_input(
 value: string,
 monaco: typeof Monaco,
 model: Monaco.editor.ITextModel
): Promise<boolean> {
 if (monaco == null || model == null) {
  return false;
 }

 const response = await try_fetch("http://127.0.0.1:8000/validate", value);
 monaco.editor.setModelMarkers(model, "owner", response.errors);

 return response.errors.length === 0;
}

interface ExecuteResponse {
 type:
  | "line"
  | "bar"
  | "pie"
  | "doughnut"
  | "radar"
  | "polarArea"
  | "bubble"
  | "scatter";
 data: {
  labels: string[] | number[];
  datasets: {
   label: string;
   data: number[];
  }[];
 };
}

export async function execute(
 value: string,
 executionArgs: string = ""
): Promise<ExecuteResponse> {
 let payload = {
  program: value,
  args: executionArgs,
 };

 const response = await try_fetch(
  "http://127.0.0.1:8000/execute",
  JSON.stringify(payload)
 );

 if (response instanceof Error) {
  throw response;
 }

 //TODO: Format response data to chartjs

 let resp: ExecuteResponse = {
  type: "line",
  data: example_data,
 };

 return resp;
}
