import test from 'node:test'
import assert from 'node:assert/strict'
import { execFileSync } from 'node:child_process'
import { mkdirSync, mkdtempSync, rmSync, symlinkSync, unlinkSync, writeFileSync } from 'node:fs'
import { tmpdir } from 'node:os'
import { join } from 'node:path'
import { sourceDigest } from './source.mts'
import { verifyReceipt } from './receipt.mts'

test('receipt exclusion cannot hide executable source or a symlink; source edits change the digest', () => {
  const previous = process.cwd(), root = mkdtempSync(join(tmpdir(), 'crowsi-source-admission-'))
  const git = (...args: string[]) => execFileSync('git', args, { stdio: 'pipe' })
  try {
    process.chdir(root); git('init', '--quiet')
    writeFileSync('source.txt', 'synthetic source'); git('add', 'source.txt')
    const before = sourceDigest()
    mkdirSync('acceptance/evidence', { recursive: true })
    const receipt = `acceptance/evidence/${'a'.repeat(40)}.json`
    writeFileSync(receipt, '{}'); git('add', receipt)
    assert.equal(sourceDigest(), before)
    writeFileSync(receipt, ' '.repeat(16_385))
    assert.throws(() => verifyReceipt(), /bounded regular file/)
    writeFileSync(receipt, '{}')
    writeFileSync('acceptance/evidence/helper.mts', 'export {}'); git('add', 'acceptance/evidence/helper.mts')
    assert.throws(() => sourceDigest())
    git('rm', '--force', 'acceptance/evidence/helper.mts')
    unlinkSync(receipt); symlinkSync('../../source.txt', receipt); git('add', receipt)
    assert.throws(() => sourceDigest())
    git('rm', '--force', receipt)
    writeFileSync('source.txt', 'changed source'); git('add', 'source.txt')
    assert.notEqual(sourceDigest(), before)
  } finally { process.chdir(previous); rmSync(root, { recursive: true, force: true }) }
})
