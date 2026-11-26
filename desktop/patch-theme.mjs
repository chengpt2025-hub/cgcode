// cgcode 界面主题：在底座设计令牌之后追加一层 cgcode 配色(iOS 蓝主按钮、iOS 风格更大的圆角)。
// 只覆盖变量，不改组件结构，风险最小；浅色/深色两套都覆盖。
import { readFileSync, writeFileSync, existsSync } from 'node:fs'
import { join } from 'node:path'

const SRC = process.argv[2]
const MARK = '/* cgcode-theme */'
const css = `
${MARK}
:root {
  --dsw-radius-xs: 6px;
  --dsw-radius-sm: 10px;
  --dsw-radius-md: 14px;
  --dsw-radius-lg: 18px;
  --dsw-radius-xl: 22px;
  --dsw-radius-panel: 30px;
}
body {
  --cgcode-ios-blue: rgb(0, 122, 255);
  --dsw-alias-button-primary-fill: var(--cgcode-ios-blue);
  --dsw-alias-button-primary-hover: rgb(0, 102, 214);
  --dsw-alias-brand-primary-new-colorprimary-new-color: var(--cgcode-ios-blue);
}
body[data-ds-dark-theme] {
  --cgcode-ios-blue: rgb(10, 132, 255);
  --dsw-alias-button-primary-fill: var(--cgcode-ios-blue);
  --dsw-alias-button-primary-hover: rgb(64, 156, 255);
  --dsw-alias-brand-primary-new-colorprimary-new-color: var(--cgcode-ios-blue);
}
body[data-ds-dark-theme] [class*="primary"]:not(:disabled) {
  --dsw-alias-label-primary-foreground: rgb(255, 255, 255);
}
`
let n = 0
for (const rel of ['packages/client/ui-theme/src/styles/design-platform.css', 'packages/client/ui-theme/src/styles/base.css']) {
  const p = join(SRC, rel)
  if (!existsSync(p)) continue
  const s = readFileSync(p, 'utf8')
  if (s.includes(MARK)) { n++; break }
  writeFileSync(p, s + css); n++; break
}
console.log(n ? 'patch-theme: 已追加 cgcode 配色(iOS 蓝、圆角)' : 'patch-theme: 未找到主题样式文件，跳过')
