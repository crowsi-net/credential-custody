import test from 'node:test'
import assert from 'node:assert/strict'
import { commands, successfulTests } from './commands.mts'
import { validateReceipt } from './receipt.mts'
import { notExecuted } from './profile.mts'
const digest = 'a'.repeat(64)
const valid = () => ({ schema: 'crowsi-local-acceptance-v1', sourceCommit: 'b'.repeat(40), sourceSha256: digest,
  executedAt: '2026-10-16T00:00:00Z', platform: 'linux', kernel: 'synthetic-kernel', notExecuted,
  results: commands.map((argv, index) => ({ argv: [...argv] as string[], exitCode: 0, logSha256: 'c'.repeat(64), passed: index >= 2 ? 4 : null })) })
test('local receipt rejects stale source and incomplete or changed checks', () => {
  validateReceipt(valid(), digest)
  assert.throws(() => validateReceipt(valid(), 'd'.repeat(64)))
  const failed = valid(); failed.results[0]!.exitCode = 1
  assert.throws(() => validateReceipt(failed, digest))
  const omitted = valid(); omitted.results.pop()
  assert.throws(() => validateReceipt(omitted, digest))
  const changed = valid(); changed.results[2]!.argv.push('--ignored')
  assert.throws(() => validateReceipt(changed, digest))
  const empty = valid(); empty.results[2]!.passed = 0
  assert.throws(() => validateReceipt(empty, digest))
  assert.throws(() => validateReceipt({ ...valid(), hostname: 'synthetic' }, digest))
  assert.throws(() => validateReceipt({ ...valid(), notExecuted: [] }, digest))
})
test('test evidence rejects missing, ignored, filtered and empty runs', () => {
  const summary = 'test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out'
  assert.equal(successfulTests(summary), 4)
  for (const bad of ['', summary.replace('0 ignored', '1 ignored'), summary.replace('0 filtered', '1 filtered'), summary.replace('4 passed', '0 passed')]) {
    assert.throws(() => successfulTests(bad))
  }
})
