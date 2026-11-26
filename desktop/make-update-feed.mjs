// 根据构建出的安装包生成 electron-updater 更新清单 stable.yml(未签名打包时 electron-builder 不会生成)。
import { createHash } from 'node:crypto'
import { readFileSync, readdirSync, statSync, writeFileSync } from 'node:fs'
import { join } from 'node:path'
const dist = process.argv[2]
const exe = readdirSync(dist).find(f => /^cgcode-.+-win-x64.*\.exe$/i.test(f))
if (!exe) { console.log('make-update-feed: 没有找到安装包，跳过'); process.exit(0) }
const m = exe.match(/^cgcode-(.+?)-win-x64/i)
const version = m[1]
const buf = readFileSync(join(dist, exe))
const sha512 = createHash('sha512').update(buf).digest('base64')
const size = statSync(join(dist, exe)).size
const yml = [
  `version: ${version}`,
  'files:',
  `  - url: ${exe}`,
  `    sha512: ${sha512}`,
  `    size: ${size}`,
  `path: ${exe}`,
  `sha512: ${sha512}`,
  `releaseDate: '${new Date().toISOString()}'`,
  '',
].join('\n')
writeFileSync(join(dist, 'stable.yml'), yml)
console.log(`make-update-feed: 已生成 stable.yml(版本 ${version}，${size} 字节)`)
