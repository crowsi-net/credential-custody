import { closeSync, constants, fstatSync, openSync, readdirSync, readFileSync } from 'node:fs'
import { commands } from './commands.mts'
import { sourceDigest } from './source.mts'
import { notExecuted } from './profile.mts'

export interface Receipt {
  schema: 'crowsi-local-acceptance-v1'; sourceCommit: string; sourceSha256: string
  executedAt: string; platform: 'linux'; kernel: string
  notExecuted: typeof notExecuted
  results: { argv: readonly string[]; exitCode: 0; logSha256: string; passed: number | null }[]
}
export function validateReceipt(value: unknown, digest: string): asserts value is Receipt {
  const r = value as Receipt
  if (!r || r.schema !== 'crowsi-local-acceptance-v1' || r.platform !== 'linux' ||
      Object.keys(r).sort().join(',') !== 'executedAt,kernel,notExecuted,platform,results,schema,sourceCommit,sourceSha256' ||
      JSON.stringify(r.notExecuted) !== JSON.stringify(notExecuted) ||
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
    const fd = openSync(`acceptance/evidence/${name}`, constants.O_RDONLY | constants.O_NOFOLLOW)
    let value: unknown
    try {
      const stat = fstatSync(fd)
      if (!stat.isFile() || stat.size > 16_384) throw Error('Receipt must be a bounded regular file')
      value = JSON.parse(readFileSync(fd, 'utf8'))
    } finally { closeSync(fd) }
    const candidate = value as Receipt
    if (candidate.sourceSha256 !== digest) continue
    validateReceipt(value, digest)
    if (name !== `${value.sourceCommit}.json`) throw Error('Receipt/source identity mismatch')
    return name
  }
  throw Error('No local execution receipt matches this source; run locally before push')
}
