import type * as Monaco from "monaco-editor/esm/vs/editor/editor.api";

export const validate_input = async (
 value: string,
 monaco: typeof Monaco,
 model: Monaco.editor.ITextModel
) => {
 if (monaco == null || model == null) {
  return;
 }

 const res = await fetch("http://127.0.0.1:8000/validate", {
  method: "POST",
  body: value,
 });
 const response = await res.json();

 monaco.editor.setModelMarkers(model, "owner", response.errors);

 console.log("response", response);
};

function setErrors(errors: Monaco.editor.IMarkerData[]) {}
