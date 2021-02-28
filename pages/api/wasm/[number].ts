import { apiBenchmark } from "@/src/benchmarks/api-benchmark";
import { calculateMaximumWealthWithWasm } from "@/src/benchmarks/maximum-wealth";

const wasmBenchmark = apiBenchmark(calculateMaximumWealthWithWasm);

export default wasmBenchmark;
