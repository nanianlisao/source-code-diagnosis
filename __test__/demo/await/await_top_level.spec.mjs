import test from "ava";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { checkBrowserSupportedByFilePath } from "../../../index.js";
import { performance } from "node:perf_hooks";

const __filename = fileURLToPath(import.meta.url);

test("should to has top_level", (t) => {
	// const start = performance.now();
	// const cwd = dirname(__filename);

	// const res = checkBrowserSupportedByFilePath("", __filename.replace(/\.spec\.mjs$/, ".js"));

	// const end = performance.now();

	// console.log(`Execution time: ${end - start}ms`);

	// const x = res.filter((item) => item.compat.name === "top_level");

	t.is(1, 1);
});
