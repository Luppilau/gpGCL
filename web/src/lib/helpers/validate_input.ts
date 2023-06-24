import { toast } from "@zerodevx/svelte-toast";
import type * as Monaco from "monaco-editor/esm/vs/editor/editor.api";

export const validate_input = async (
 value: string,
 monaco: typeof Monaco,
 model: Monaco.editor.ITextModel
) => {
 if (monaco == null || model == null) {
  return;
 }
 try {
  const res = await fetch("http://127.0.0.1:8000/validate", {
   method: "POST",
   body: value,
  });

  if (res.status !== 200) {
   toast.push("Could not connect to server", { target: "top" });
   return;
  }
  const response = await res.json();
  console.log(response.errors);
  monaco.editor.setModelMarkers(model, "owner", response.errors);
 } catch (error) {
  toast.push("Could not connect to server", { target: "top" });
 }
};
