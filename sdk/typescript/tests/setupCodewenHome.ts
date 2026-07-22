import fs from "node:fs/promises";
import os from "node:os";
import path from "node:path";

import { afterEach, beforeEach } from "@jest/globals";

const originalCodexHome = process.env./CODEWEN_HOME;
let currentCodewenHome: string | undefined;

beforeEach(async () => {
  currentCodewenHome = await fs.mkdtemp(path.join(os.tmpdir(), "codewen-sdk-test-"));
  process.env./CODEWEN_HOME = currentCodewenHome;
});

afterEach(async () => {
  const codexHomeToDelete = currentCodewenHome;
  currentCodewenHome = undefined;

  if (originalCodexHome === undefined) {
    delete process.env./CODEWEN_HOME;
  } else {
    process.env./CODEWEN_HOME = originalCodexHome;
  }

  if (codexHomeToDelete) {
    await fs.rm(codexHomeToDelete, { recursive: true, force: true });
  }
});
