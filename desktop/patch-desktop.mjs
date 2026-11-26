// 首次欢迎页：API Key 改存为商汤日日新(SENSENOVA_API_KEY)，文案改为日日新；DeepSeek 账号登录保留为可选。
import { readFileSync, writeFileSync } from 'node:fs'
const D = process.argv[2] + '/apps/desktop/src'
function patch(file, pairs) {
  let s = readFileSync(file, 'utf8')
  for (const [from, to, label, all] of pairs) {
    if (!s.includes(from)) { console.error(`patch-desktop: 未找到 ${label}`); process.exit(1) }
    s = all ? s.split(from).join(to) : s.replace(from, to)
    console.log(`patch-desktop: 已改 ${label}`)
  }
  writeFileSync(file, s)
}
patch(`${D}/welcome-backend.ts`, [[
  "    const official: unknown = settings.namespaces.find((item: unknown) => record(item) && item.ns === 'llm-deepseek')\n    if (official === undefined) return { settings: { namespaces: settings.namespaces }, ref: undefined }\n    if (!record(official) || !record(official.value) || typeof official.value.apiKeyEnv !== 'string') {\n      throw new Error('desktop welcome: missing official DeepSeek credential reference')\n    }\n    return { settings: { namespaces: settings.namespaces }, ref: official.value.apiKeyEnv }\n",
  "    // cgcode：欢迎页填写的 Key 存为商汤日日新凭据，默认模型即走该路由。\n    return { settings: { namespaces: settings.namespaces }, ref: 'SENSENOVA_API_KEY' as string | undefined }\n",
  '欢迎页凭据改为 SENSENOVA_API_KEY']])
patch(`${D}/locale.ts`, [
  ["welcomeKeyDescription: 'Configure official DeepSeek models to start using Harness'", "welcomeKeyDescription: 'Enter your SenseNova API Key to start using cgcode'", '英文欢迎文案'],
  ["welcomeKeyDescription: '配置 DeepSeek 官方模型，即可开始使用'", "welcomeKeyDescription: '填写商汤日日新 API Key，即可开始使用'", '中文欢迎文案'],
])

// 首页鲸鱼装饰(DeepSeek 吉祥物)不再显示
patch(`${process.argv[2]}/packages/client/ui-brand-official/src/client/index.ts`, [[
  "  ctx.slots.inject('conversation.hero.frame', () => ctx.slots.register({\n    name: 'conversation.hero.frame',\n    id: 'pixel-whale',\n  }, PixelWhale))\n",
  "  void PixelWhale\n", '去掉首页鲸鱼装饰']])
