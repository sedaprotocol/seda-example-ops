// biome-ignore assist/source/organizeImports: biome is lying
import { expect } from 'bun:test';
import { AbiCoder as _AbiCoder } from 'ethers';
import type { HttpFetchResponseData, VmResult } from '@seda-protocol/vm';

export const AbiCoder = _AbiCoder.defaultAbiCoder();

type RevealResult = {
  exitCode: number;
  gasUsed: number;
  inConsensus: boolean;
  result: Buffer;
};

function genericCreateSuccessfulReveal(result: Buffer): RevealResult {
  return {
    exitCode: 0,
    gasUsed: 0,
    inConsensus: true,
    result,
  };
}

function createSuccessfulJsonBigIntArrayReveal(values: bigint[]): RevealResult {
  const encoded = `[${values.map((v) => v.toString()).join(',')}]`;
  return genericCreateSuccessfulReveal(Buffer.from(encoded));
}

function createSuccessfulJsonBigIntReveal(value: bigint): RevealResult {
  const buf = Buffer.alloc(16);
  buf.writeBigUInt64LE(value & ((1n << 64n) - 1n), 0);
  buf.writeBigUInt64LE(value >> 64n, 8);
  const jsonBytes = Buffer.from(JSON.stringify(Array.from(buf)));
  return genericCreateSuccessfulReveal(jsonBytes);
}

function createSuccessfulBigIntReveal(value: bigint): RevealResult {
  const buf = Buffer.alloc(16);
  buf.writeBigUInt64LE(value & ((1n << 64n) - 1n), 0);
  buf.writeBigUInt64LE(value >> 64n, 8);
  return genericCreateSuccessfulReveal(buf);
}

function createSuccessfulHttpFetchResponseReveal(data: HttpFetchResponseData): RevealResult {
  const jsonBytes = Buffer.from(JSON.stringify(data));
  return genericCreateSuccessfulReveal(jsonBytes);
}

function createFailedReveal(): RevealResult {
  return {
    exitCode: 1,
    gasUsed: 0,
    inConsensus: false,
    result: Buffer.from('Error while fetching symbol prices'),
  };
}

export enum RevealKind {
  JsonBigIntArray,
  JsonBigInt,
  BigInt,
  Failed,
  HttpFetchResponse,
}

export type RevealInput =
  | [RevealKind.Failed]
  | [RevealKind.BigInt, bigint]
  | [RevealKind.JsonBigInt, bigint]
  | [RevealKind.JsonBigIntArray, bigint[]]
  | [RevealKind.HttpFetchResponse, HttpFetchResponseData];

export function createRevealArray(values: RevealInput[]): RevealResult[] {
  return values.map(([kind, val]) => {
    switch (kind) {
      case RevealKind.Failed:
        return createFailedReveal();
      case RevealKind.BigInt:
        return createSuccessfulBigIntReveal(val as bigint);
      case RevealKind.JsonBigInt:
        return createSuccessfulJsonBigIntReveal(val as bigint);
      case RevealKind.JsonBigIntArray:
        return createSuccessfulJsonBigIntArrayReveal(val as bigint[]);
      case RevealKind.HttpFetchResponse:
        return createSuccessfulHttpFetchResponseReveal(val as HttpFetchResponseData);
    }
  });
}

export function makeDataProxyResponse(url: string, obj: unknown, publickey?: string, signature?: string) {
  // error if obj is not an Object
  if (typeof obj !== 'object' || obj === null) {
    throw new Error('Invalid object');
  }

  const json = JSON.stringify(obj);
  const bytes = new TextEncoder().encode(json);
  const res = new Response(bytes, {
    status: 200,
    headers: {
      'x-seda-publickey': publickey || '02ee9686b002e8f57f9a2ca7089a6b587c9ef4e6c2b67159add5151a42ce5e6668',
      'x-seda-signature':
        signature ||
        '20fab238a55e7e09353c3a7f7903987035e692923d0419514491191501a793f3675bbec89bc4a61f9bc03eef4bdb3147002d39d15a0b47686ca2fecd66134578',
    },
  });
  Object.defineProperty(res, 'url', { value: url });
  Object.defineProperty(res, 'size', { value: bytes.length });
  return res;
}

function genericHandleTallyVmResult<T>(vmResult: VmResult, exitCode: number, expected: T, codec?: string) {
  console.debug('Stdout:', vmResult.stdout);
  console.debug('Stderr:', vmResult.stderr);
  if (vmResult.exitCode !== 0) {
    console.error('Result:', Buffer.from(vmResult.result).toString('utf-8'));
  }
  expect(vmResult.exitCode).toBe(exitCode);

  // Decode the result using ethers' AbiCoder
  if (vmResult.exitCode === 0 && codec) {
    const [bnArray] = AbiCoder.decode([codec], vmResult.result) as unknown as [T];
    expect(bnArray).toEqual(expected);
  }
}

export function handleJsonBigIntExecutionVmResult(vmResult: VmResult, exitCode: number, expected: bigint) {
  genericHandleTallyVmResult(vmResult, exitCode, expected);
  // convert vmResult.result from bytes of json(serde_json::to_vec) to a buffer
  const jsonString = Buffer.from(vmResult.result).toString('utf-8');
  expect(jsonString).toBeDefined();
  expect(jsonString.length).toBeGreaterThan(0);
  const jsonArray = JSON.parse(jsonString);
  // convert Uint8Array of 16bytes(u128) to bigint from le_bytes
  const buf = Buffer.from(jsonArray);
  expect(buf.length).toBe(16);
  const value = BigInt.asUintN(128, BigInt(buf.readBigUInt64LE(0)) + (BigInt(buf.readBigUInt64LE(8)) << 64n));
  expect(value).toBe(expected);
}

export function handleJsonArrayBigIntExecutionVmResult(vmResult: VmResult, exitCode: number, expected: bigint[]) {
  genericHandleTallyVmResult(vmResult, exitCode, expected);
  // convert vmResult.result from bytes of json(serde_json::to_vec) to a buffer
  const jsonString = Buffer.from(vmResult.result).toString('utf-8');
  expect(jsonString).toBeDefined();
  expect(jsonString.length).toBeGreaterThan(0);
  const jsonArray = JSON.parse(jsonString).map((v) => BigInt(v));
  expect(jsonArray).toEqual(expected);
}

export function handleBigIntExecutionVmResult(vmResult: VmResult, exitCode: number, expected: bigint) {
  genericHandleTallyVmResult(vmResult, exitCode, expected);
  // convert Uint8Array of 16bytes(u128) to bigint from le_bytes
  const buf = Buffer.from(vmResult.result);
  expect(buf.length).toBe(16);
  const value = BigInt.asUintN(128, BigInt(buf.readBigUInt64LE(0)) + (BigInt(buf.readBigUInt64LE(8)) << 64n));
  expect(value).toBe(expected);
}

export function handleHttpFetchResponseExecutionVmResult(
  vmResult: VmResult,
  exitCode: number,
  expected: HttpFetchResponseData,
) {
  genericHandleTallyVmResult(vmResult, exitCode, expected);

  // convert vmResult.result from bytes to HttpFetchResponse
  const response = JSON.parse(Buffer.from(vmResult.result).toString('utf-8')) as HttpFetchResponseData;
  expect(response).toEqual(expected);
}

export function handleBigIntTallyVmResult(vmResult: VmResult, exitCode: number, expected: bigint) {
  genericHandleTallyVmResult(vmResult, exitCode, expected, 'uint256');
}

export function handleBigIntArrayTallyVmResult(vmResult: VmResult, exitCode: number, expected: bigint[]) {
  genericHandleTallyVmResult(vmResult, exitCode, expected, 'uint256[]');
}

export function handleJsonBigIntArrayExecutionVmResult(vmResult: VmResult, exitCode: number, expected: bigint[]) {
  genericHandleTallyVmResult(vmResult, exitCode, expected);

  // Parse JSON array of bigint strings
  const jsonString = Buffer.from(vmResult.result).toString('utf-8');
  expect(jsonString).toBeDefined();
  expect(jsonString.length).toBeGreaterThan(0);

  const jsonArray = JSON.parse(jsonString);
  expect(Array.isArray(jsonArray)).toBe(true);

  // Convert each string to bigint and compare
  const values = jsonArray.map((v: string) => BigInt(v));
  expect(values).toEqual(expected);
}
