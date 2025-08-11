// biome-ignore assist/source/organizeImports: biome is lying
import { expect } from 'bun:test';
import { AbiCoder as _AbiCoder } from 'ethers';
import type { VmResult } from '@seda-protocol/vm';

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

export function createSuccessfulJsonBigIntArrayReveal(values: bigint[]): RevealResult {
  const encoded = `[${values.map((v) => v.toString()).join(',')}]`;
  return genericCreateSuccessfulReveal(Buffer.from(encoded));
}

export function createSuccessfulJsonBigIntReveal(value: bigint): RevealResult {
  const buf = Buffer.alloc(16);
  buf.writeBigUInt64LE(value & ((1n << 64n) - 1n), 0);
  buf.writeBigUInt64LE(value >> 64n, 8);
  const jsonBytes = Buffer.from(JSON.stringify(Array.from(buf)));
  return genericCreateSuccessfulReveal(jsonBytes);
}

export function createSuccessfulBigIntReveal(value: bigint): RevealResult {
  const buf = Buffer.alloc(16);
  buf.writeBigUInt64LE(value & ((1n << 64n) - 1n), 0);
  buf.writeBigUInt64LE(value >> 64n, 8);
  return genericCreateSuccessfulReveal(buf);
}

export function createFailedReveal(): {
  exitCode: number;
  gasUsed: number;
  inConsensus: boolean;
  result: Buffer;
} {
  return {
    exitCode: 1,
    gasUsed: 0,
    inConsensus: false,
    result: Buffer.from('Error while fetching symbol prices'),
  };
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

export function handleBigIntExecutionVmResult(vmResult: VmResult, exitCode: number, expected: bigint) {
  genericHandleTallyVmResult(vmResult, exitCode, expected);
  // convert Uint8Array of 16bytes(u128) to bigint from le_bytes
  const buf = Buffer.from(vmResult.result);
  expect(buf.length).toBe(16);
  const value = BigInt.asUintN(128, BigInt(buf.readBigUInt64LE(0)) + (BigInt(buf.readBigUInt64LE(8)) << 64n));
  expect(value).toBe(expected);
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
