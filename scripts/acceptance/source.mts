import { createHash } from 'node:crypto'
import { execFileSync } from 'node:child_process'
import { lstatSync } from 'node:fs'

export const git = (...args: string[]) => execFileSync('git', args, { encoding: 'utf8' }).trim()
export function sourceDigest(reference?: string): string {
  const listing = reference ? git('ls-tree', '-r', '-z', '--full-tree', reference) : git('ls-files', '--stage', '-z')
  const files = listing.split('\0').filter(Boolean).map(row => {
    const tab = row.indexOf('\t'), header = row.slice(0, tab).split(' '), path = row.slice(tab + 1)
    return { path, mode: header[0], object: header[reference ? 2 : 1] }
  }).sort((a, b) => a.path < b.path ? -1 : a.path > b.path ? 1 : 0)
  const hash = createHash('sha256')
  for (const { path, mode, object } of files) {
    if (!['100644', '100755'].includes(mode ?? '') || (!reference && !lstatSync(path).isFile())) {
      throw Error('Acceptance source must contain regular files')
    }
    if (path.startsWith('acceptance/evidence/')) {
      if (!/^acceptance\/evidence\/[a-f0-9]{40}\.json$/.test(path)) throw Error('Only named JSON receipts may be excluded from source')
      continue
    }
    hash.update(`${path}\0${mode}\0${object}\0`)
  }
  return hash.digest('hex')
}
export function clean(): void {
  if (git('status', '--porcelain', '--untracked-files=all')) throw Error('Commit source/evidence first; worktree must be clean')
}
