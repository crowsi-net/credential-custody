import { execFileSync } from 'node:child_process'
const agent = 'crowsi-device-credential-agent'
const run = (args: string[]) => execFileSync('cargo', ['+1.97', ...args], { stdio: 'inherit' })
run(['test', '--locked', '--workspace', '--exclude', agent, '--all-targets', '--all-features'])
run(['test', '--locked', '-p', agent, '--lib', '--bins', '--examples', '--all-features'])
const metadata = JSON.parse(execFileSync('cargo', ['+1.97', 'metadata', '--locked', '--no-deps', '--format-version', '1'], { encoding: 'utf8' })) as {
  packages: { name: string; targets: { name: string; kind: string[] }[] }[]
}
const targets = metadata.packages.find(p => p.name === agent)?.targets.filter(t => t.kind.includes('test'))
if (!targets?.some(t => t.name === 'production_bootstrap')) throw Error('Local acceptance target missing')
for (const target of targets.filter(t => t.name !== 'production_bootstrap')) {
  run(['test', '--locked', '-p', agent, '--test', target.name, '--all-features'])
}
console.log('Portable targets executed. production_bootstrap NOT RUN here; requires separately reviewed local evidence.')
