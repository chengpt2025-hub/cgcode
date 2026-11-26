// 正式版版本号：把底座整套 package.json 里与根目录一致的版本号统一改为 cgcode 正式版版本号。
// 底座各包版本号必须一致(workspace 约束)，所以整体替换而不是只改桌面端。
import { readFileSync, writeFileSync, readdirSync, statSync } from 'node:fs'
import { join } from 'node:path'

const SRC = process.argv[2]
const TARGET = process.argv[3] || '1.0.0'
const rootPkg = JSON.parse(readFileSync(join(SRC, 'package.json'), 'utf8'))
const base = rootPkg.version
if (!base) { console.log('patch-version: 根 package.json 无版本号，跳过'); process.exit(0) }
let n = 0
function walk(dir) {
  for (const name of readdirSync(dir)) {
    if (name === 'node_modules' || name === '.git' || name.startsWith('.desktop-build')) continue
    const p = join(dir, name)
    let st; try { st = statSync(p) } catch { continue }
    if (st.isDirectory()) walk(p)
    else if (name === 'package.json') {
      const s = readFileSync(p, 'utf8')
      const re = new RegExp(`("version"\\s*:\\s*)"${base.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')}"`)
      if (re.test(s)) { writeFileSync(p, s.replace(re, `$1"${TARGET}"`)); n++ }
    }
  }
}
walk(SRC)
console.log(`patch-version: ${base} -> ${TARGET}，共改 ${n} 个 package.json`)
