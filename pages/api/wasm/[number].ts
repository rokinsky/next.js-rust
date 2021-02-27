import { apiBenchmark } from "@/src/api-benchmark";
import { calculateMaximumWealthWithWasm } from "@/src/maximum-wealth";

const wasmBenchmark = apiBenchmark(calculateMaximumWealthWithWasm);

export default wasmBenchmark;
