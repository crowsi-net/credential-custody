import { execFileSync } from 'node:child_process'
import { notExecuted } from './profile.mts'
const agent = 'crowsi-device-credential-agent'
const windows = notExecuted[0].package
const run = (args: string[]) => execFileSync('cargo', ['+1.97', ...args], { stdio: 'inherit' })
run(['test', '--locked', '--workspace', '--exclude', agent, '--exclude', windows, '--all-targets', '--all-features'])
const metadata = JSON.parse(execFileSync('cargo', ['+1.97', 'metadata', '--locked', '--no-deps', '--format-version', '1'], { encoding: 'utf8' })) as {
  packages: { name: string; targets: { name: string; kind: string[] }[] }[]
}
for (const [name, separate] of [[agent, 'production_bootstrap'], [windows, notExecuted[0].target]]) {
  run(['test', '--locked', '-p', name!, '--lib', '--bins', '--examples', '--all-features'])
  const targets = metadata.packages.find(p => p.name === name)?.targets.filter(t => t.kind.includes('test'))
  if (!targets?.some(t => t.name === separate)) throw Error('Declared separate acceptance target missing')
  for (const target of targets.filter(t => t.name !== separate)) {
    run(['test', '--locked', '-p', name!, '--test', target.name, '--all-features'])
  }
}
console.log('Portable targets executed. production_bootstrap NOT RUN by this command; see separate local execution evidence.')
console.log('NOT EXECUTED: windows-dpapi-live requires a Windows PE and interactive DPAPI profile; no Windows acceptance claimed.')
