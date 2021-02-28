export const maximumWealth = (accounts: number[][]): number => {
  return accounts
    .map((account) => account.reduce((acc, cur) => acc + cur, 0))
    .reduce((max, cur) => Math.max(max, cur), Number.MIN_SAFE_INTEGER);
};

export const calculateMaximumWealthWithJs = (length: number): number => {
  const accounts = Array.from({ length }, () =>
    Array.from({ length }, (v, k) => k)
  );
  return maximumWealth(accounts);
};

export const calculateMaximumWealthWithWasm = async (
  length: number
): Promise<number> => {
  const wasm = await import("@/crate/pkg/wasm");
  return wasm.calculate_maximum_wealth(length);
};
