
// biome-ignore assist/source/organizeImports: biome is lying
import { file } from "bun";
import { afterEach, describe, expect, it, mock } from "bun:test";
import { AbiCoder } from "ethers";
import {
	testOracleProgramExecution,
	testOracleProgramTally,
} from "@seda-protocol/dev-tools";

const WASM_PATH = "target/wasm32-wasip1/release/single-price-feed.wasm";

const fetchMock = mock();

afterEach(() => {
	fetchMock.mockRestore();
});

describe("data request execution", () => {
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

	it("tally works with one reveal", async () => {
		const oracleProgram = await file(WASM_PATH).arrayBuffer();

		// Result from the execution test
		const buffer = Buffer.from([
			91, 49, 49, 56, 48, 53, 55, 48, 48, 48, 48, 48, 48, 44, 51, 55, 56, 51,
			52, 48, 48, 48, 48, 48, 93,
		]);
		const vmResult = await testOracleProgramTally(
			Buffer.from(oracleProgram),
			Buffer.from("tally-inputs"),
			[
				{
					exitCode: 0,
					gasUsed: 0,
					inConsensus: true,
					result: buffer,
				},
			],
		);

		console.debug("Stdout:", vmResult.stdout);
		console.debug("Stderr:", vmResult.stderr);
		if (vmResult.exitCode !== 0) {
			console.error("Result:", Buffer.from(vmResult.result).toString("utf-8"));
		}
		expect(vmResult.exitCode).toBe(0);

		const coder = AbiCoder.defaultAbiCoder();
		const [bnArray] = coder.decode(
			["uint256[]"],
			vmResult.result,
		) as unknown as [bigint[]];
		expect(bnArray.length).toBe(2);
		expect(bnArray[0]).toBe(118057000000n);
		expect(bnArray[1]).toBe(3783400000n);
	});

	it("should correctly compute the median from multiple reveals", async () => {
		const oracleProgram = await file(WASM_PATH).arrayBuffer();

		const results = [
			{
				exitCode: 0,
				gasUsed: 0,
				inConsensus: true,
				// (btc, eth) = (100, 20)
				result: Buffer.from([91, 49, 48, 48, 44, 50, 48, 93]),
			},
			{
				exitCode: 0,
				gasUsed: 0,
				inConsensus: true,
				// (btc, eth) = (200, 30)
				result: Buffer.from([91, 50, 48, 48, 44, 51, 48, 93]),
			},
		];

		const vmResult = await testOracleProgramTally(
			Buffer.from(oracleProgram),
			Buffer.from("tally-inputs"),
			results,
		);

		console.debug("Stdout:", vmResult.stdout);
		console.debug("Stderr:", vmResult.stderr);
		if (vmResult.exitCode !== 0) {
			console.error("Result:", Buffer.from(vmResult.result).toString("utf-8"));
		}
		expect(vmResult.exitCode).toBe(0);

		const coder = AbiCoder.defaultAbiCoder();
		const [bnArray] = coder.decode(
			["uint256[]"],
			vmResult.result,
		) as unknown as [bigint[]];
		expect(bnArray.length).toBe(2);
		expect(bnArray[0]).toBe(150n);
		expect(bnArray[1]).toBe(25n);
	});
});
