// 自动更新改指向 cgcode 自己的更新源(GitHub Pages)，避免客户端去 DeepSeek 官方服务器检查更新。
import { readFileSync, writeFileSync } from 'node:fs'
import { join } from 'node:path'
const src = process.argv[2]
const f = join(src, 'apps/desktop-publisher/src/desktop-auto-update-environment.mjs')
let t = readFileSync(f, 'utf8')
const pairs = [
  ["fixedOrigin: 'https://download.deepseek.com',", "fixedOrigin: 'https://chengpt2025-hub.github.io',", '更新源改为 GitHub Pages'],
  ["let releasePrefix = 'dsh-desk'", "let releasePrefix = 'cgcode/updates'", '更新路径改为 /cgcode/updates'],
]
for (const [a, b, label] of pairs) {
  if (!t.includes(a)) { console.error(`patch-update: 未找到 ${label}`); process.exit(1) }
  t = t.replace(a, b); console.log(`patch-update: 已改 ${label}`)
}
writeFileSync(f, t)
