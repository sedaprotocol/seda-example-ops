// biome-ignore assist/source/organizeImports: biome is lying
import { file } from 'bun';
import { afterEach, describe, it, mock } from 'bun:test';
import {
  // testOracleProgramExecution,
  testOracleProgramTally,
} from '@seda-protocol/dev-tools';
import {
  createSuccessfulBigIntReveal as createSuccessfulReveal,
  createFailedReveal,
  handleBigIntVmResult as handleVmResult,
} from './utils.js';

const WASM_PATH = 'target/wasm32-wasip1/release/single-commodity-price.wasm';

const fetchMock = mock();

afterEach(() => {
  fetchMock.mockRestore();
});

describe('single commodity price', () => {
  it('works with 1 price', async () => {
    const oracleProgram = await file(WASM_PATH).arrayBuffer();
    const vmResult = await testOracleProgramTally(Buffer.from(oracleProgram), Buffer.from('tally-inputs'), [
      createSuccessfulReveal(100n),
    ]);
    handleVmResult(vmResult, 0, 100n);
  });

  it('works with 2 prices', async () => {
    const oracleProgram = await file(WASM_PATH).arrayBuffer();
    const vmResult = await testOracleProgramTally(Buffer.from(oracleProgram), Buffer.from('tally-inputs'), [
      createSuccessfulReveal(100n),
      createSuccessfulReveal(200n),
    ]);
    handleVmResult(vmResult, 0, 150n);
  });

  it('works with 5 prices', async () => {
    const oracleProgram = await file(WASM_PATH).arrayBuffer();
    const vmResult = await testOracleProgramTally(Buffer.from(oracleProgram), Buffer.from('tally-inputs'), [
      createSuccessfulReveal(100n),
      createSuccessfulReveal(200n),
      createSuccessfulReveal(300n),
      createSuccessfulReveal(400n),
      createSuccessfulReveal(500n),
    ]);
    handleVmResult(vmResult, 0, 300n);
  });

  it('works with 10 prices', async () => {
    const oracleProgram = await file(WASM_PATH).arrayBuffer();
    const vmResult = await testOracleProgramTally(Buffer.from(oracleProgram), Buffer.from('tally-inputs'), [
      createSuccessfulReveal(100n),
      createSuccessfulReveal(200n),
      createSuccessfulReveal(300n),
      createSuccessfulReveal(400n),
      createSuccessfulReveal(500n),
      createSuccessfulReveal(600n),
      createSuccessfulReveal(700n),
      createSuccessfulReveal(800n),
      createSuccessfulReveal(900n),
      createSuccessfulReveal(1000n),
    ]);
    handleVmResult(vmResult, 0, 550n);
  });

  describe('tally works with errored executions', () => {
    it('should ignore the errored execution', async () => {
      const oracleProgram = await file(WASM_PATH).arrayBuffer();

      const vmResult = await testOracleProgramTally(Buffer.from(oracleProgram), Buffer.from('tally-inputs'), [
        createSuccessfulReveal(100n),
        createFailedReveal(),
        createSuccessfulReveal(200n),
      ]);

      handleVmResult(vmResult, 0, 150n);
    });

    it('should ignore multiple errored executions', async () => {
      const oracleProgram = await file(WASM_PATH).arrayBuffer();

      const vmResult = await testOracleProgramTally(Buffer.from(oracleProgram), Buffer.from('tally-inputs'), [
        createSuccessfulReveal(100n),
        createFailedReveal(),
        createSuccessfulReveal(200n),
        createFailedReveal(),
        createSuccessfulReveal(300n),
      ]);

      handleVmResult(vmResult, 0, 200n);
    });

    it('should error if all executions errored', async () => {
      const oracleProgram = await file(WASM_PATH).arrayBuffer();
      const vmResult = await testOracleProgramTally(Buffer.from(oracleProgram), Buffer.from('tally-inputs'), [
        createFailedReveal(),
        createFailedReveal(),
        createFailedReveal(),
      ]);

      handleVmResult(vmResult, 1, 0n);
    });
  });
});
