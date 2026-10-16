import { createHash } from 'node:crypto'
import { execFileSync, spawnSync } from 'node:child_process'
import { mkdirSync, writeFileSync } from 'node:fs'
import { release } from 'node:os'
import { commands, successfulTests } from './commands.mts'
import { clean, git, sourceDigest } from './source.mts'
import { verifyReceipt, type Receipt } from './receipt.mts'

const mode = process.argv[2]
if (process.argv.length !== 3 || !['run', 'verify', 'push'].includes(mode ?? '')) throw Error('Usage: local.mts run|verify|push')
clean()
if (mode === 'run') {
  if (process.platform !== 'linux' || process.getuid?.() === 0) throw Error('Use a non-root Linux account; no sudo or host changes')
  const receipt: Receipt = {
    schema: 'crowsi-local-acceptance-v1', sourceCommit: git('rev-parse', 'HEAD'), sourceSha256: sourceDigest(),
    executedAt: new Date().toISOString(), platform: 'linux', kernel: release(), results: []
  }
  mkdirSync('.local/acceptance', { recursive: true, mode: 0o700 })
  for (const [index, argv] of commands.entries()) {
    const result = spawnSync(argv[0], [...argv.slice(1)], { encoding: 'utf8', maxBuffer: 32 * 1024 * 1024,
      timeout: 45 * 60 * 1000, env: { ...process.env, CARGO_BUILD_JOBS: '2', CARGO_INCREMENTAL: '0' } })
    const output = `${result.stdout ?? ''}${result.stderr ?? ''}`
    writeFileSync(`.local/acceptance/${receipt.sourceCommit}-${index}.log`, output, { mode: 0o600 })
    if (result.error || result.status !== 0) throw Error(`Local check ${index} failed; inspect private log; no receipt or push`)
    receipt.results.push({ argv, exitCode: 0, logSha256: createHash('sha256').update(output).digest('hex'),
      passed: index >= 2 ? successfulTests(output) : null })
    console.log(`Local check ${index + 1}/${commands.length} passed`)
  }
  clean()
  if (sourceDigest() !== receipt.sourceSha256) throw Error('Source changed during acceptance')
  mkdirSync('acceptance/evidence', { recursive: true })
  writeFileSync(`acceptance/evidence/${receipt.sourceCommit}.json`, JSON.stringify(receipt, null, 2) + '\n', { flag: 'wx' })
  console.log('Commit the receipt, review its source binding, then run local.mts push. Nothing pushed.')
} else {
  console.log(`Matching local receipt: ${verifyReceipt()}; execution claim requires maintainer review`)
  if (mode === 'push') execFileSync('git', ['push', 'origin', 'HEAD'], { stdio: 'inherit' })
}
