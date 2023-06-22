import type * as Monaco from "monaco-editor/esm/vs/editor/editor.api";

export const language: Monaco.languages.ILanguageExtensionPoint = {
 id: "gpgcl",
 extensions: ["gpgcl"],
 aliases: [],
 mimetypes: ["application/gpgcl"],
};

export const configuration: Monaco.languages.LanguageConfiguration = {
 brackets: [
  ["(", ")"],
  ["{", "}"],
  ["[", "]"],
 ],
 autoClosingPairs: [
  { open: "(", close: ")" },
  { open: "{", close: "}" },
  { open: "[", close: "]" },
 ],
 surroundingPairs: [
  { open: "(", close: ")" },
  { open: "{", close: "}" },
  { open: "[", close: "]" },
 ],
};

export const tokenProvider: Monaco.languages.IMonarchLanguage = {
 defaultToken: "",
 tokenPostfix: ".gpgcl",

 brackets: [
  { token: "delimiter.parenthesis", open: "(", close: ")" },
  { token: "delimiter.curly", open: "{", close: "}" },
  { token: "delimiter.square", open: "[", close: "]" },
 ],

 keywords: [
  "skip",
  "diverge",
  "tick",
  "if",
  "else",
  "while",
  "normal",
  "uniform",
  "lognormal",
  "exponential",
 ],

 operators: [
  ":=",
  "*",
  "/",
  "%",
  "+",
  "-",
  ":-",
  "!",
  "&&",
  "||",
  "<",
  "<=",
  ">",
  ">=",
  "==",
  "!=",
 ],

 // we include these common regular expressions	symbols: /[=><!~?:&|+\-*\/\^%]+/,
 symbols: /[=><!~?:&|+\-*\/\^%]+/,
 escapes:
  /\\(?:[abfnrtv\\"']|x[0-9A-Fa-f]{1,4}|u[0-9A-Fa-f]{4}|U[0-9A-Fa-f]{8})/,
 digits: /\d+(_+\d+)*/,
 octaldigits: /[0-7]+(_+[0-7]+)*/,
 binarydigits: /[0-1]+(_+[0-1]+)*/,
 hexdigits: /[[0-9a-fA-F]+(_+[0-9a-fA-F]+)*/,

 // The main tokenizer for our languages
 tokenizer: {
  root: [
   // identifiers and keywords
   [
    /[a-zA-Z_$][\w$]*/,
    {
     cases: {
      "@keywords": { token: "keyword.$0" },
      "@default": "identifier",
     },
    },
   ],

   // delimiters and operators
   [/[{}()\[\]]/, "@brackets"],
   [/[](?!@symbols)/, "@brackets"],
   [
    /@symbols/,
    {
     cases: {
      "@operators": "delimiter",
      "@default": "",
     },
    },
   ],
   [/(@digits)[fFdD]/, "number.float"],
   [/(@digits)[lL]?/, "number"],
  ],
 },
};
