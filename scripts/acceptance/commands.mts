export const commands = [
  ['cargo', '+1.97', 'fmt', '--all', '--', '--check'],
  ['cargo', '+1.97', 'clippy', '--locked', '--workspace', '--all-targets', '--all-features', '--', '-D', 'warnings'],
  ['cargo', '+1.97', 'test', '--locked', '-p', 'crowsi-device-credential-agent', '--test', 'production_bootstrap', '--all-features'],
  ['node', '--experimental-strip-types', 'scripts/acceptance/portable.mts'],
  ['cargo', '+1.97', 'test', '--locked', '--workspace', '--doc', '--all-features'],
  ['bash', 'scripts/acceptance/linux-keyring.sh']
] as const

export function successfulTests(output: string): number {
  const summaries = [...output.matchAll(/test result: ok\. (\d+) passed; (\d+) failed; (\d+) ignored; (\d+) measured; (\d+) filtered out/g)]
  if (!summaries.length || summaries.some(row => row.slice(2).some(value => Number(value) !== 0))) {
    throw Error('Missing, failed, ignored, measured or filtered test evidence')
  }
  const passed = summaries.reduce((total, row) => total + Number(row[1]), 0)
  if (passed === 0) throw Error('No tests executed')
  return passed
}
