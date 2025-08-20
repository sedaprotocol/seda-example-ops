// // biome-ignore assist/source/organizeImports: biome is lying
// import { file } from 'bun';
// import { afterEach, describe, it, mock } from 'bun:test';
// import { testOracleProgramExecution, testOracleProgramTally } from '@seda-protocol/dev-tools';
// import {
//   handleBigIntArrayTallyVmResult as handleVmResult,
//   handleHttpFetchResponseExecutionVmResult as handleExecutionVmResult,
//   createRevealArray,
//   RevealKind,
//   makeDataProxyResponse,
// } from './utils.js';

// const WASM_PATH = 'target/wasm32-wasip1/release/single-equity-price-verification.wasm';

// const fetchMock = mock();

// afterEach(() => {
//   fetchMock.mockRestore();
// });

// describe('single price feed', () => {
//   describe('execution phase', () => {
//     it('works', async () => {
//       fetchMock.mockImplementation((url) => {
//         return makeDataProxyResponse(
//           url,
//           { bitcoin: { usd: 121239 }, ethereum: { usd: 4658.03 } },
//           '02ee9686b002e8f57f9a2ca7089a6b587c9ef4e6c2b67159add5151a42ce5e6668',
//           '20fab238a55e7e09353c3a7f7903987035e692923d0419514491191501a793f3675bbec89bc4a61f9bc03eef4bdb3147002d39d15a0b47686ca2fecd66134578',
//         );
//       });

//       const oracleProgram = await file(WASM_PATH).arrayBuffer();

//       const vmResult = await testOracleProgramExecution(
//         Buffer.from(oracleProgram),
//         Buffer.from('BTC,ETH'),
//         fetchMock,
//         undefined,
//         undefined,
//         undefined,
//         0n,
//       );

//       const response = JSON.stringify({
//         bitcoin: { usd: 121239 },
//         ethereum: { usd: 4658.03 },
//       });
//       const response_bytes: number[] = Array.from(Buffer.from(response));
//       handleExecutionVmResult(vmResult, 0, {
//         status: 200,
//         headers: {
//           'x-seda-signature':
//             '20fab238a55e7e09353c3a7f7903987035e692923d0419514491191501a793f3675bbec89bc4a61f9bc03eef4bdb3147002d39d15a0b47686ca2fecd66134578',
//           'x-seda-publickey': '02ee9686b002e8f57f9a2ca7089a6b587c9ef4e6c2b67159add5151a42ce5e6668',
//         },
//         url: 'http://34.78.7.237:5384/proxy/usd/BTC,ETH',
//         bytes: response_bytes,
//         content_length: response_bytes.length,
//       });
//     });
//   });

//   describe('tally phase', () => {
//     it('works', async () => {
//       const response = JSON.stringify({
//         btc: { usd: 113301 },
//         eth: { usd: 4151.3 },
//       });
//       const response_bytes: number[] = Array.from(Buffer.from(response));
//       const oracleProgram = await file(WASM_PATH).arrayBuffer();

//       const vmResult = await testOracleProgramTally(
//         Buffer.from(oracleProgram),
//         Buffer.from('tally-inputs'),
//         createRevealArray([
//           [
//             RevealKind.HttpFetchResponse,
//             {
//               status: 200,
//               headers: {
//                 'x-seda-signature':
//                   '4aeb96020f97296ae7c5eb9b5c39edaf026a1ec19fbcd0733b97f1268e3e10d55ff73f9803234afc491180cb5db9bf0830a4f3be6d9dd41080fe290eb5daafeb',
//                 'x-seda-publickey': '02ee9686b002e8f57f9a2ca7089a6b587c9ef4e6c2b67159add5151a42ce5e6668',
//               },
//               url: 'http://34.78.7.237:5384/proxy/usd/BTC,ETH',
//               bytes: response_bytes,
//               content_length: response_bytes.length,
//             },
//           ],
//         ]),
//       );

//       handleVmResult(vmResult, 0, [113301000000n, 4151300000n]);
//     });

//     describe('works with errored executions', () => {
//       it('should error if all executions errored', async () => {
//         const oracleProgram = await file(WASM_PATH).arrayBuffer();
//         const vmResult = await testOracleProgramTally(
//           Buffer.from(oracleProgram),
//           Buffer.from('tally-inputs'),
//           createRevealArray([[RevealKind.Failed]]),
//         );

//         handleVmResult(vmResult, 1, [0n]);
//       });
//     });
//   });
// });
