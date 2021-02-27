import { apiBenchmark } from "@/src/api-benchmark";
import { calculateMaximumWealthWithJs } from "@/src/maximum-wealth";

const jsBenchmark = apiBenchmark(calculateMaximumWealthWithJs);

export default jsBenchmark;
