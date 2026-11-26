// 把底座自带、默认关闭的实用插件改成新安装时默认开启。
import { readFileSync, writeFileSync } from 'node:fs'
import { join } from 'node:path'
const src = process.argv[2]
const f = join(src, 'packages/util/official-bundles/src/bundles.ts')
let t = readFileSync(f, 'utf8')
const add = [
  '@deepseek-ai/dsh-experimental-voice-input-bundle',
  '@deepseek-ai/dsh-experimental-session-search',
  '@deepseek-ai/dsh-experimental-session-titles-bundle',
  '@deepseek-ai/dsh-experimental-plugin-market-bundle',
]
const before = "bundles: ['@deepseek-ai/dsh-base', '@deepseek-ai/dsh-web-app'],"
if (!t.includes(before)) throw new Error('patch-defaults: web 模板结构变化')
t = t.replace(before, `bundles: ['@deepseek-ai/dsh-base', '@deepseek-ai/dsh-web-app', ${add.map(a => `'${a}'`).join(', ')}],`)
writeFileSync(f, t)
console.log('patch-defaults: 默认开启 ' + add.length + ' 个底座插件')
