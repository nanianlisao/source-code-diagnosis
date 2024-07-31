import test from "ava";
import { demo } from "../../index.js";
import { fileURLToPath } from "node:url";
import { dirname, join } from "node:path";
import { performance } from "node:perf_hooks";

// 获取当前文件的路径
const __filename = fileURLToPath(import.meta.url);

test("getModuleMemberUsage", (t) => {
	const start = performance.now();
	const cwd = dirname(__filename);

	const res = demo(["antd"], {
		cwd: join(cwd, "f1"),
		concurrency: 2,
	});
	const end = performance.now();
	console.log(`Execution time: ${end - start}ms`);

	console.log(res);
	t.is(1, 1);
});
