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

// 开机视频：主窗口出现前播放 cgcode 开机视频(原画质)，点击/按键跳过，最长 15 秒
patch(`${D}/main.ts`, [[
  "async function main(): Promise<void> {\n",
  `function keepCgcodeAliveDuringIntro(): void {}

async function playCgcodeIntro(): Promise<void> {
  if (!app.isPackaged || process.env.CGCODE_NO_INTRO !== undefined) return
  const page = join(process.resourcesPath, 'intro.html')
  try { await access(page) } catch { return }
  // 修复闪退：开机视频窗口此时是唯一的窗口。Electron 在没有任何 window-all-closed 监听时，
  // 会在所有窗口关闭后自动退出程序；而底座自己的监听要等主流程后面才注册。
  // 所以视频一结束(窗口关闭)程序就开始退出，约 3 秒后整个进程消失。这里先挂一个空监听，
  // 阻止这个默认退出；之后底座注册的监听照常生效(关掉主窗口仍会正常退出)。
  app.on('window-all-closed', keepCgcodeAliveDuringIntro)
  await new Promise<void>((resolve) => {
    const intro = new BrowserWindow({
      width: 1248, height: 704, frame: false, resizable: false, center: true, show: false,
      backgroundColor: '#000000', skipTaskbar: true, alwaysOnTop: true,
      webPreferences: { autoplayPolicy: 'no-user-gesture-required', sandbox: true, contextIsolation: true },
    })
    const finish = (): void => { clearTimeout(timer); if (!intro.isDestroyed()) intro.destroy(); resolve() }
    const timer = setTimeout(finish, 16000)
    intro.on('closed', finish)
    intro.once('ready-to-show', () => { intro.show() })
    intro.loadFile(page).catch(finish)
  })
}

async function main(): Promise<void> {
  await playCgcodeIntro().catch(() => undefined)
`, '开机视频']])
patch(`${process.argv[2]}/apps/desktop/scripts/electron-builder-config.mjs`, [[
  "      { from: fileURLToPath(new URL('../resources/icon-windows.png', import.meta.url)), to: 'icon.png' },\n",
  "      { from: fileURLToPath(new URL('../resources/icon-windows.png', import.meta.url)), to: 'icon.png' },\n      { from: fileURLToPath(new URL('../resources/intro.mp4', import.meta.url)), to: 'intro.mp4' },\n      { from: fileURLToPath(new URL('../resources/intro.html', import.meta.url)), to: 'intro.html' },\n",
  '打包开机视频']])

// 预装中文技能：首次启动把 resources/skills 复制到 ~/.agents/skills(已存在的不覆盖)
patch(`${D}/main.ts`, [
  ["import { access, readFile, writeFile } from 'node:fs/promises'\n",
   "import { access, readFile, writeFile, cp as cgcodeCopy, mkdir as cgcodeMkdir } from 'node:fs/promises'\nimport { homedir as cgcodeHomedir } from 'node:os'\n", '技能复制所需导入'],
  ["async function main(): Promise<void> {\n",
   `async function installCgcodeSkills(): Promise<void> {
  if (!app.isPackaged) return
  const source = join(process.resourcesPath, 'skills')
  try { await access(source) } catch { return }
  const target = join(process.env.DSH_AGENTS_HOME ?? join(cgcodeHomedir(), '.agents'), 'skills')
  await cgcodeMkdir(target, { recursive: true })
  await cgcodeCopy(source, target, { recursive: true, force: false, errorOnExist: false })
}

async function main(): Promise<void> {
  await installCgcodeSkills().catch(() => undefined)
`, '预装技能']])
patch(`${process.argv[2]}/apps/desktop/scripts/electron-builder-config.mjs`, [[
  "      { from: fileURLToPath(new URL('../resources/intro.html', import.meta.url)), to: 'intro.html' },\n",
  "      { from: fileURLToPath(new URL('../resources/intro.html', import.meta.url)), to: 'intro.html' },\n      { from: fileURLToPath(new URL('../resources/skills', import.meta.url)), to: 'skills' },\n",
  '打包预装技能']])

// ===== 与已安装的 DeepSeek Harness 彻底隔离(防止两者抢同一套目录、锁和协议) =====
// 1) 底座的 Electron 用户目录取自包名 @deepseek-ai/dsh-desktop，和原版 DSH 完全相同(%APPDATA%\@deepseek-ai\dsh-desktop)，
//    导致两者共用单实例锁、设置、日志：DSH 在托盘运行时 cgcode 一启动就退出。
// 2) 数据目录 ~/.dsh(配置、凭据、会话、插件、桌面锁 ~/.dsh/profiles/desktop/lock)两者共用，会互相覆盖、抢锁。
// 3) 运行时把 dsh:// 协议注册给自己，会抢走原版 DSH 的链接。
// 做法：在任何路径被读取之前，把用户目录改到 %APPDATA%\cgcode，把数据目录改到 ~/.cgcode，协议改为 cgcode://。
patch(`${D}/main.ts`, [
  ["let focusPrimaryWindow = (): void => {}\n",
   `// cgcode：与原版 DeepSeek Harness 隔离，必须在读取任何用户目录/数据目录之前执行
if (app.isPackaged || process.env.CGCODE_ISOLATE === '1') {
  const cgcodeHome = process.env.CGCODE_HOME?.trim() || join(cgcodeHomedir(), '.cgcode')
  // 无论系统里是否设置过 DSH_HOME(原版留下的)，cgcode 一律用自己的数据目录；子进程会继承这个值
  process.env.DSH_HOME = cgcodeHome
  app.setName('cgcode')
  app.setPath('userData', join(app.getPath('appData'), 'cgcode'))
  if (process.platform === 'win32') app.setAppUserModelId('com.cgcode.desktop')
}
let focusPrimaryWindow = (): void => {}
`, '与 DSH 隔离：用户目录、数据目录、任务栏标识'],
  ["app.setAsDefaultProtocolClient('dsh')", "app.setAsDefaultProtocolClient('cgcode')", '协议改为 cgcode://(不再抢 dsh://)'],
  ["if (url === 'dsh://open' || url === 'dsh://open/') focusPrimaryWindow()",
   "if (url === 'cgcode://open' || url === 'cgcode://open/') focusPrimaryWindow()", '协议打开地址'],
])
// 兜底：底座各处默认数据目录名 .dsh 改为 .cgcode(即使某个子进程没拿到 DSH_HOME 也不会落到原版目录)
patch(`${process.argv[2]}/packages/util/home-paths/src/index.ts`, [[
  "export const DSH_HOME_DIR_NAME = '.dsh'", "export const DSH_HOME_DIR_NAME = '.cgcode'", '默认数据目录名 .cgcode']])
// 以后的签名版自动更新：下载缓存目录不再和原版共用 @deepseek-aidsh-desktop-updater
patch(`${process.argv[2]}/apps/desktop/scripts/electron-builder-config.mjs`, [[
  "[{ provider: 'generic', url: update.publicUrl, channel: update.channel }]",
  "[{ provider: 'generic', url: update.publicUrl, channel: update.channel, updaterCacheDirName: 'cgcode-updater' }]",
  '更新缓存目录独立']])
