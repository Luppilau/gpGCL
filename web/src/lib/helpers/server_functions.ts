import { toast } from "@zerodevx/svelte-toast";
import { example_data } from "$lib/examples/example-data";
import type * as Monaco from "monaco-editor/esm/vs/editor/editor.api";
import type { ChartType } from "chart.js";

const SERVER_URL = "http://localhost:8000/";

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

 const response = await try_fetch(SERVER_URL + "/validate", value);
 monaco.editor.setModelMarkers(model, "owner", response.errors);

 return response.errors.length === 0;
}

interface ExecuteResponse {
 type: ChartType;
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

 return new Promise(async (resolve, reject) => {
  const response = await try_fetch(
   SERVER_URL + "/execute",
   JSON.stringify(payload)
  );

  if (response instanceof Error) {
   reject(response);
  }

  if (response.errors.length > 0 && response.errors[0] !== "") {
   reject(response.errors);
  }

  resolve(response.result);
 });
}
