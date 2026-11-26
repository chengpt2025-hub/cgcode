// 构建时拉取开源热门技能(固定提交)，补中文说明，放进 apps/desktop/resources/skills。
import { readFileSync, writeFileSync, cpSync, existsSync, mkdirSync, rmSync, readdirSync, copyFileSync } from 'node:fs'
import { execFileSync } from 'node:child_process'
import { join } from 'node:path'
const [root, src, work] = process.argv.slice(2)
const m = JSON.parse(readFileSync(join(root, 'desktop/upstream-skills.json'), 'utf8'))
const dest = join(src, 'apps/desktop/resources/skills')
mkdirSync(dest, { recursive: true })
for (const [repo, sha] of Object.entries(m.repos)) {
  const dir = join(work, repo.replace('/', '_'))
  rmSync(dir, { recursive: true, force: true })
  execFileSync('git', ['init', '-q', dir])
  execFileSync('git', ['-C', dir, 'remote', 'add', 'origin', `https://github.com/${repo}.git`])
  execFileSync('git', ['-C', dir, 'fetch', '-q', '--depth', '1', 'origin', sha])
  execFileSync('git', ['-C', dir, 'checkout', '-q', 'FETCH_HEAD'])
}
let n = 0
for (const [name, s] of Object.entries(m.skills)) {
  const from = join(work, s.repo.replace('/', '_'), s.path)
  const to = join(dest, name)
  rmSync(to, { recursive: true, force: true })
  cpSync(from, to, { recursive: true })
  const f = join(to, 'SKILL.md')
  let t = readFileSync(f, 'utf8')
  t = t.replace(/^description:\s*(.*)$/m, (_, d) => {
    let o = d.trim(); if (/^["']/.test(o)) o = o.slice(1, -1)
    return 'description: ' + JSON.stringify(`${s.zh}。${o}`)
  })
  writeFileSync(f, t)
  if (!readdirSync(to).some(x => /^license/i.test(x))) {
    const lic = join(work, s.repo.replace('/', '_'), 'LICENSE')
    if (existsSync(lic)) copyFileSync(lic, join(to, 'LICENSE.txt'))
  }
  n++
}
console.log(`fetch-skills: 已预装 ${n} 个开源技能`)
