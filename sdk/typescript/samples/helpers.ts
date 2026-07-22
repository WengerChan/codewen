import path from "node:path";

export function codewenPathOverride() {
  return (
    process.env.CODEX_EXECUTABLE ??
    path.join(process.cwd(), "..", "..", "codewen-rs", "target", "debug", "codewen")
  );
}
