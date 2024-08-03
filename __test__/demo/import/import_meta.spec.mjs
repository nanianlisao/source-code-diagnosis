import { fileURLToPath } from "node:url";
import { test } from "../common.mjs";

const __filename = fileURLToPath(import.meta.url);

test("import_meta", __filename);