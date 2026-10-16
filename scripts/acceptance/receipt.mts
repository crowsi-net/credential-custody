import { readdirSync, readFileSync } from 'node:fs'
import { commands } from './commands.mts'
import { git, sourceDigest } from './source.mts'

export interface Receipt {
  schema: 'crowsi-local-acceptance-v1'; sourceCommit: string; sourceSha256: string
  executedAt: string; platform: 'linux'; kernel: string
  results: { argv: readonly string[]; exitCode: 0; logSha256: string; passed: number | null }[]
}
export function validateReceipt(value: unknown, digest: string): asserts value is Receipt {
  const r = value as Receipt
  if (!r || r.schema !== 'crowsi-local-acceptance-v1' || r.platform !== 'linux' ||
      Object.keys(r).sort().join(',') !== 'executedAt,kernel,platform,results,schema,sourceCommit,sourceSha256' ||
      !/^[a-f0-9]{40}$/.test(r.sourceCommit) || r.sourceSha256 !== digest ||
      !Number.isFinite(Date.parse(r.executedAt)) || typeof r.kernel !== 'string' || !/^[a-z0-9._+-]{1,120}$/i.test(r.kernel) ||
      !Array.isArray(r.results) || r.results.length !== commands.length) throw Error('Invalid/stale local receipt')
  r.results.forEach((row, index) => {
    if (!row || Object.keys(row).sort().join(',') !== 'argv,exitCode,logSha256,passed' ||
        row.exitCode !== 0 || JSON.stringify(row.argv) !== JSON.stringify(commands[index]) ||
        !/^[a-f0-9]{64}$/.test(row.logSha256) || (index >= 2
          ? !Number.isSafeInteger(row.passed) || Number(row.passed) <= 0 : row.passed !== null)) {
      throw Error('Incomplete or changed local acceptance command')
    }
  })
}
export function verifyReceipt(): string {
  const digest = sourceDigest()
  const files = readdirSync('acceptance/evidence').filter(name => /^[a-f0-9]{40}\.json$/.test(name))
  for (const name of files) {
    const value: unknown = JSON.parse(readFileSync(`acceptance/evidence/${name}`, 'utf8'))
    const candidate = value as Receipt
    if (candidate.sourceSha256 !== digest) continue
    validateReceipt(value, digest)
    if (name !== `${value.sourceCommit}.json`) throw Error('Receipt/source identity mismatch')
    git('merge-base', '--is-ancestor', value.sourceCommit, 'HEAD')
    if (sourceDigest(value.sourceCommit) !== digest) throw Error('Claimed source commit has different content')
    return name
  }
  throw Error('No local execution receipt matches this source; run locally before push')
}
