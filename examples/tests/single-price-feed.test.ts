// biome-ignore assist/source/organizeImports: biome is lying
import { file } from 'bun';
import { afterEach, describe, it, mock, test } from 'bun:test';
import {
  // testOracleProgramExecution,
  testOracleProgramTally,
} from '@seda-protocol/dev-tools';
import {
  createSuccessfulBigIntArrayReveal as createSuccessfulReveal,
  createFailedReveal,
  handleBigIntArrayVmResult as handleVmResult,
} from './utils.js';

const WASM_PATH = 'target/wasm32-wasip1/release/single-price-feed.wasm';

const fetchMock = mock();

afterEach(() => {
  fetchMock.mockRestore();
});

describe('single price feed', () => {
  // it("should return the correct prices", async () => {
  // 	fetchMock.mockImplementation((_) => {
  // 		return new Response(
  // 			JSON.stringify({
  // 				btc: { usd: 118027 },
  // 				eth: { usd: 3782.64 },
  // 			}),
  // 		);
  // 	});

  // 	const oracleProgram = await file(WASM_PATH).arrayBuffer();

  // 	const vmResult = await testOracleProgramExecution(
  // 		Buffer.from(oracleProgram),
  // 		Buffer.from("BTC,ETH"),
  // 		fetchMock,
  // 	);

  // 	console.debug("Stdout:", vmResult.stdout);
  // 	console.debug("Stderr:", vmResult.stderr);
  // 	if (vmResult.exitCode !== 0) {
  // 		console.error("Result:", Buffer.from(vmResult.result).toString("utf-8"));
  // 	}

  // 	expect(vmResult.exitCode).toBe(0);
  // 	const json = new TextDecoder().decode(vmResult.result);
  // 	const prices = JSON.parse(json) as number[]; // or `as string[]` then map to BigInt
  // 	expect(prices).toEqual([
  // 		118027000000, // Bitcoin
  // 		3782640000, // Ethereum
  // 	]);
  // });

  describe('tally works with 1 price', () => {
    const cases = [
      {
        inputs: [[100n]],
        expected: [100n],
      },
      {
        inputs: [[100n], [200n]],
        expected: [150n],
      },
      {
        inputs: [[100n], [200n], [300n], [400n], [500n], [600n], [700n], [800n], [900n]],
        expected: [500n],
      },
      {
        inputs: [[100n], [200n], [300n], [400n], [500n], [600n], [700n], [800n], [900n], [1000n]],
        expected: [550n],
      },
    ];

    cases.forEach(({ inputs, expected }) => {
      test(`with ${inputs.length} reveals`, async () => {
        const oracleProgram = await file(WASM_PATH).arrayBuffer();

        const vmResult = await testOracleProgramTally(
          Buffer.from(oracleProgram),
          Buffer.from('tally-inputs'),
          inputs.map((vals) => createSuccessfulReveal(vals)),
        );

        handleVmResult(vmResult, 0, expected);
      });
    });
  });

  describe('tally works with 5 prices', () => {
    const cases = [
      {
        inputs: [[100n, 200n, 300n, 400n, 500n]],
        expected: [100n, 200n, 300n, 400n, 500n],
      },
      {
        inputs: [
          [100n, 200n, 300n, 400n, 500n],
          [200n, 300n, 400n, 500n, 600n],
        ],
        expected: [150n, 250n, 350n, 450n, 550n],
      },
      {
        inputs: [
          [100n, 200n, 300n, 400n, 500n],
          [200n, 300n, 400n, 500n, 600n],
          [300n, 400n, 500n, 600n, 700n],
          [400n, 500n, 600n, 700n, 800n],
          [500n, 600n, 700n, 800n, 900n],
        ],
        expected: [300n, 400n, 500n, 600n, 700n],
      },
      {
        inputs: [
          [100n, 200n, 300n, 400n, 500n],
          [200n, 300n, 400n, 500n, 600n],
          [300n, 400n, 500n, 600n, 700n],
          [400n, 500n, 600n, 700n, 800n],
          [500n, 600n, 700n, 800n, 900n],
          [600n, 700n, 800n, 900n, 1000n],
        ],
        expected: [350n, 450n, 550n, 650n, 750n],
      },
    ];

    cases.forEach(({ inputs, expected }) => {
      test(`with ${inputs.length} reveals and ${inputs[0].length} prices`, async () => {
        const oracleProgram = await file(WASM_PATH).arrayBuffer();

        const vmResult = await testOracleProgramTally(
          Buffer.from(oracleProgram),
          Buffer.from('tally-inputs'),
          inputs.map((vals) => createSuccessfulReveal(vals)),
        );

        handleVmResult(vmResult, 0, expected);
      });
    });
  });

  describe('tally works with errored executions', () => {
    it('should ignore the errored execution', async () => {
      const oracleProgram = await file(WASM_PATH).arrayBuffer();

      const vmResult = await testOracleProgramTally(Buffer.from(oracleProgram), Buffer.from('tally-inputs'), [
        createSuccessfulReveal([100n]),
        createFailedReveal(),
        createSuccessfulReveal([200n]),
      ]);

      handleVmResult(vmResult, 0, [150n]);
    });

    it('should ignore multiple errored executions', async () => {
      const oracleProgram = await file(WASM_PATH).arrayBuffer();

      const vmResult = await testOracleProgramTally(Buffer.from(oracleProgram), Buffer.from('tally-inputs'), [
        createSuccessfulReveal([100n]),
        createFailedReveal(),
        createSuccessfulReveal([200n]),
        createFailedReveal(),
        createSuccessfulReveal([300n]),
      ]);

      handleVmResult(vmResult, 0, [200n]);
    });

    it('should error if all executions errored', async () => {
      const oracleProgram = await file(WASM_PATH).arrayBuffer();
      const vmResult = await testOracleProgramTally(Buffer.from(oracleProgram), Buffer.from('tally-inputs'), [
        createFailedReveal(),
        createFailedReveal(),
        createFailedReveal(),
      ]);

      handleVmResult(vmResult, 1, [0n]);
    });
  });
});
