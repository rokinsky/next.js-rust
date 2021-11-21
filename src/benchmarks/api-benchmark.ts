import { NextApiHandler } from "next";

export type ApiBenchmark = (
  fn: (length: number) => number | Promise<number>
) => NextApiHandler;

export const apiBenchmark: ApiBenchmark = (fn) => async (req, res) => {
  const startTime = Date.now();
  const {
    query: { number },
  } = req;

  const parsedNumber = Number(number) || 0;
  const result = await fn(Math.max(parsedNumber, 1));

  const time = `${(Date.now() - startTime) / 1000}s`;
  res.status(200).json({ result, time });
};
