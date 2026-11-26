// 安全加固：把底座依赖里有已公开漏洞(pnpm audit，2026-11-27)且会随客户端发布的包，
// 通过 pnpm overrides 升到同一大版本内的已修复版本。测试/发布工具专用的依赖不随安装包发布，不在此列。
import { readFileSync, writeFileSync } from 'node:fs'
import { join } from 'node:path'

const SRC = process.argv[2]
const p = join(SRC, 'pnpm-workspace.yaml')
let s = readFileSync(p, 'utf8')
const MARK = '  # cgcode-security-overrides'
if (s.includes(MARK)) { console.log('patch-security: 已存在，跳过'); process.exit(0) }
const add = [
  MARK,
  "  '@modelcontextprotocol/client@2': '2.4.0'",   // 重定向时泄露凭据(中危)
  "  '@modelcontextprotocol/server@2': '2.4.0'",   // 单请求耗尽资源、令牌校验(高危)
  "  '@modelcontextprotocol/sdk@1': '1.32.2'",     // 同上(高危)
  "  'hono@4': '4.13.22'",                         // 路径解码绕过、中间件绕过(中危)
  "  '@hono/node-server@1': '1.19.19'",            // 反斜杠路径绕过(中危)
  "  'body-parser@2': '2.4.0'",                    // 拒绝服务(高危)
  "  'fast-uri@3': '3.1.9'",                       // URI 欺骗(中危)
  "  'fflate@0.8': '0.8.3'",                       // 恶意 ZIP 死循环(中危)
  "  'ip-address@10': '10.7.6'",                   // 正则拒绝服务(中危)
  "  'qs@6': '6.17.1'",                            // 未捕获异常(中危)
].join('\n')
if (!/^overrides:\s*$/m.test(s)) { console.log('patch-security: 未找到 overrides，跳过'); process.exit(0) }
s = s.replace(/^overrides:\s*$/m, m => `${m}\n${add}`)
writeFileSync(p, s)
console.log('patch-security: 已加 10 条依赖安全覆盖')
