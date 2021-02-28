import { apiBenchmark } from "@/src/benchmarks/api-benchmark";
import { calculateMaximumWealthWithJs } from "@/src/benchmarks/maximum-wealth";

const jsBenchmark = apiBenchmark(calculateMaximumWealthWithJs);

export default jsBenchmark;
