<div align="center">

<img src="https://raw.githubusercontent.com/chengpt2025-hub/cgcode/v2/desktop/overlay/apps/desktop/resources/icon.png" width="120" alt="cgcode logo" />

# cgcode

### 내 모델로 움직이는, 중국어 사용자를 위한 AI 코딩 에이전트 & 올인원 데스크톱 어시스턴트

[简体中文](README.md) | [繁體中文](README.zh-TW.md) | [English](README.en.md) | [日本語](README.ja.md) | **한국어**

[다운로드(터미널판)](https://github.com/chengpt2025-hub/cgcode/releases) · [핵심 특징](#핵심-특징) · [cgcode를 선택하는 이유](#cgcode를-선택하는-이유) · [로드맵](#로드맵) · [자주 묻는 질문](#자주-묻는-질문)

</div>

---

## 한 문장 소개

cgcode는 **원하는 대규모 언어 모델로 사용할 수 있는 AI 어시스턴트**입니다. 세계 최고 수준의 오픈소스 에이전트 기반(OpenAI Codex, DeepSeek Harness)을 **완전한 중국어 지원, 설정 불필요, 계정 불필요, 원격 측정 없음**의 제품으로 재구성했습니다. 개발자를 위한 **터미널판**과 모두를 위한 **데스크톱 클라이언트**를 제공합니다.

| | 터미널판(v1) | 데스크톱 클라이언트(v2) |
|---|---|---|
| 대상 | 개발자 | 모든 사람: 학생, 직장인, 크리에이터, 팀 |
| 할 수 있는 일 | 프로젝트 이해, 코드 작성, 파일 수정, 명령 실행, 코드 리뷰, Git 관리 | 채팅, 웹 검색, 문서·스프레드시트·슬라이드 작성, 이미지 생성, 코딩, 멀티 에이전트 협업, 예약 작업 |
| 사용 방식 | 터미널에서 실행 | 더블클릭으로 설치, 창 기반 UI |
| 브랜치 | `main` | `v2` |
| 상태 | 검수판 배포 완료 | 검수판 다운로드 가능 |

---

## 핵심 특징

### 1. 모델 선택의 자유
- 기본값으로 SenseTime의 **SenseNova(日日新)** 가 연결되어 있어 바로 사용할 수 있습니다.
- DeepSeek, Qwen, Kimi 등 **OpenAI 호환** 엔드포인트와 중계 서비스를 모두 지원합니다.
- 터미널판은 **Chat Completions 브리지를 내장**합니다. 업스트림 Codex는 Chat 지원을 제거하고 Responses API만 지원하지만, cgcode는 Chat만 지원하는 모델로도 에이전트를 완전히 구동합니다.
- 데스크톱판은 UI에서 **언제든 모델을 전환**할 수 있습니다.

### 2. 진정한 중국어 우선
- UI, 슬래시 명령 설명, 환영 화면, 상태 화면, 키 안내, 기본 응답까지 모두 간체 중국어입니다.
- 기본 탑재 스킬도 중국어로 작성되어 자연스러운 중국어 표현으로 실행됩니다.

### 3. 진입 장벽 제로
- 첫 실행 시 **중국어 설정 마법사**에서 엔드포인트, 키, 모델명을 입력하면 **연결을 자동 테스트**합니다. 설정 파일을 편집할 필요가 없습니다.
- 데스크톱판은 환영 화면에 API 키 하나만 입력하면 됩니다.
- ChatGPT, Claude 등 해외 계정이 필요 없습니다.

### 4. 개인정보 우선
- API 키는 내 컴퓨터에만 저장됩니다.
- 대화는 내가 설정한 모델 서비스로만 전송됩니다.
- 사용 데이터 전송, 피드백 업로드, 세션 원격 측정, 공식 업데이트 확인은 **기본적으로 모두 꺼져 있습니다**.

### 5. 안정성
- 속도 제한(429) 발생 시 **자동으로 빠르게 재시도**합니다.
- **예비 엔드포인트**를 설정하면 기본 엔드포인트 실패 시 자동 전환합니다.

### 6. 바로 쓰는 스킬 생태계
- 터미널판은 중국어 개발 스킬 **6개**, 데스크톱판은 **38개**(자체 개발 중국어 스킬 19개 + 엄선한 인기 오픈소스 스킬 19개)를 기본 탑재합니다.
- 데스크톱판에는 **중국어 스킬 마켓**이 내장되어 있습니다. 한 문장으로 skills.sh, ClawHub, SkillHub를 검색하고 결과를 중국어로 보여 주며, **안전 점검 후 원클릭 설치**합니다.
- 모든 기본 탑재 스킬은 라이선스 확인과 보안 검토를 거쳤습니다. **탈옥(jailbreak) 계열 스킬은 거부**하여 키와 계정이 정지될 위험을 막습니다.

### 7. 아름다운 디자인
- **iOS 스타일 디자인**: 둥근 모서리, 넉넉한 여백, iOS 시스템 색상. 터미널판은 7가지 테마를 `/theme`로 실시간 미리 보며 전환합니다.
- 공식 **10초 부팅 애니메이션**: 지구 → 웜홀 → 기계 터널 → 섬광 → CG CODE / SYSTEM ONLINE, 역동적인 음악과 함께.

### 8. 올인원 + 멀티 에이전트(데스크톱판)
- 채팅, 검색, Word / Excel / PowerPoint, 이미지, 코딩을 하나의 앱에서.
- Grok을 참고한 **멀티 에이전트 협업**: 복잡한 작업을 여러 에이전트가 나누어 병렬로 처리합니다.
- **예약 작업**과 **플러그인**으로 확장할 수 있습니다.

---

## cgcode를 선택하는 이유

| | cgcode | 공식 Codex CLI | ChatGPT / Claude 앱 |
|---|---|---|---|
| 사용 가능한 모델 | OpenAI 호환 모델 전부, 기본값 SenseNova | 주로 OpenAI 모델, Responses API만 | 자사 모델만 |
| Chat만 지원하는 모델(중국 업체에 많음) | 지원(브리지 내장) | 미지원 | 미지원 |
| UI 언어 | 완전 중국어 | 영어 | 다국어 |
| 계정 | 불필요, 내 키만 있으면 됨 | OpenAI 계정 또는 키 | 해외 계정 |
| 첫 설정 | 중국어 마법사 + 연결 테스트 | 수동 설정 | 계정 로그인 |
| 사용 데이터 전송 | 기본 전부 꺼짐 | 공급사 설정에 따름 | 공급사 정책에 따름 |
| 중국어 스킬 기본 탑재 | 6개(터미널) / 38개 + 스킬 마켓(데스크톱) | 없음 | 없음 |
| 비개발자용 데스크톱 앱 | 있음 | 없음 | 있음 |

---

## 빠른 시작

### 터미널판

1. [Releases](https://github.com/chengpt2025-hub/cgcode/releases)에서 최신 `cgcode-windows-x64.zip`(약 186MB)을 내려받아 `D:\cgcode` 등에 압축을 풉니다.
2. 터미널 또는 PowerShell에서 프로젝트 폴더로 이동한 뒤 `D:\cgcode\cgcode.exe`를 실행합니다.
   - 아직 코드 서명이 없으므로 "Windows의 PC 보호" 창이 뜨면 **추가 정보 → 실행**을 클릭하세요.
3. 부팅 애니메이션(아무 키나 눌러 건너뛰기) 후 마법사에 입력합니다:
   - ① 엔드포인트(예: `https://token.sensenova.cn/v1`)
   - ② API 키
   - ③ 모델명(예: `sensenova-6.8-flash`)
4. 연결 테스트를 통과하면 바로 대화를 시작할 수 있습니다.

### 데스크톱 클라이언트(검수 단계)

1. GitHub에 로그인하고 [Actions](https://github.com/chengpt2025-hub/cgcode/actions)에서 `v2` 브랜치의 최신 성공 빌드를 엽니다.
2. **Artifacts**에서 `cgcode-windows-x64`를 내려받아 압축을 풉니다.
3. `cgcode-<버전>-win-x64-unsigned.exe`를 실행합니다. SmartScreen이 나타나면 **추가 정보 → 실행**을 클릭합니다.
4. 첫 실행 시 부팅 영상이 재생되고, 환영 화면에서 SenseNova API 키를 입력하면 사용할 수 있습니다.

> 정식판은 서명된 설치 프로그램과 자동 업데이트를 제공할 예정입니다.

---

## 터미널판 상세

### 테마(7가지, `/theme`로 전환)

| 테마 | 설명 |
|---|---|
| iOS 다크 | 기본값 |
| iOS 라이트 | 밝은 터미널 배경용 |
| 미드나이트 블루 | 짙은 파랑 |
| 민트 그린 | 산뜻한 초록 |
| 선셋 오렌지 | 따뜻한 주황 |
| 그래파이트 그레이 | 차분한 회색 |
| 고대비 | 접근성, 화면 공유용 |

각 테마는 UI 색상과 코드 하이라이트를 함께 통일합니다.

### 기본 탑재 스킬(6개)

| 스킬 | 기능 | 실행 계기 |
|---|---|---|
| cg-review | 중국어 코드 리뷰, 심각도별 문제와 수정 제안 | "리뷰해 줘", "코드 봐 줘" |
| cg-ship | 배포 전 점검: 포맷·빌드·테스트·민감 정보 검사, 중국어 커밋 메시지 작성 | "배포", "커밋 전 점검" |
| cg-focus | `.cgcode/FOCUS.md` 포커스 보드로 긴 작업 관리 | "어디까지 했지?" |
| cg-memory | `.cgcode/MEMORY.md`에 규칙과 교훈 기록 | "기억해 둬" |
| cg-git | Git 도우미: 상태 설명, 브랜치·충돌·되돌리기, 위험한 작업은 사전 확인 | git, 브랜치, 충돌 |
| cg-context | 컨텍스트 관리: 진행 상황 요약, `/compact` 또는 `/new` 제안 | "너무 길어", "요약해 줘" |

### 설정 파일

위치: `%USERPROFILE%\.cgcode\cgcode.toml`(마법사가 자동 생성, 보통 편집 불필요).

```toml
[api]
base_url = "https://token.sensenova.cn/v1"
api_key  = "내 키"
model    = "sensenova-6.8-flash"

# 선택: 기본 엔드포인트 실패 시 사용할 예비 엔드포인트
# [[fallback]]
# base_url = "https://api.deepseek.com/v1"
# api_key  = "..."
# model    = "deepseek-chat"

[intro]
enabled = true   # false로 부팅 애니메이션 끄기
sound   = true   # false로 부팅 소리 끄기
```

---

## 데스크톱 클라이언트 상세

### 내장 기능
- **모델 전환**: 기본값 SenseNova `sensenova-6.8-flash`, Qwen(qwen-plus, qwen-max)과 Kimi(moonshot-v1-128k) 사전 설정, OpenAI 호환 모델 자유 추가.
- **올인원**: 채팅, 웹 검색, Word / Excel / PowerPoint, 이미지 생성, 코딩.
- **멀티 에이전트 협업**, **예약 작업**, **플러그인 관리**.
- 원본 화질 부팅 영상(1248×704), 클릭 또는 아무 키로 건너뛰기.
- **v2.1 신규 기능**:
  - 기본 켜짐: **로컬 음성 입력**(SenseVoice, CPU에서 실행, 첫 사용 시 모델 다운로드), **대화 기록 검색**, **대화 제목 자동 생성**, **플러그인 마켓**.
  - **PC 조작**: PC의 다른 프로그램을 조작(agent-desktop 기반), 위험한 작업은 매번 확인.
  - **웹 통합 읽기**: Bilibili, YouTube 자막, V2EX, 雪球, GitHub, RSS, 모든 웹 페이지 읽기. 샤오훙수 등 로그인 채널은 기본 꺼짐, 켜기 전 계정 정지 위험 안내(Agent-Reach 기반).
  - **장기 기억**: 기억해 달라고 한 선호를 PC에 저장해 새 대화에서도 기억.
  - **안심 모드**: 전송·결제·삭제 전 반드시 확인, 비밀번호·인증 코드는 다루지 않음, 웹 내용은 자료로만 취급, 작업 후 한 일을 보고.
- **v2.2 신규 기능**:
  - **로컬 오프라인 모델**: Ollama를 설치하면 모델 목록에 표시, 인터넷과 API 키 불필요. Qwen 3.5 4B, DeepSeek R1 8B/14B 사전 설정.
  - **음성 합성·낭독**: 다양한 방언과 언어로 읽기, 목소리 복제 가능(본인 또는 허락받은 목소리만, 파일명에 "AI合成"), VoxCPM2 기반.
  - **크리에이터 스튜디오**: 기획, 대본, 콘티, 캐릭터 음성, 자막까지 한 번에.
  - **대화 통계 바**: 턴 수, 모델·도구 소요 시간, 첫 글자까지 시간.
- **정식판 1.0.0 신규 기능**:
  - **자동 업데이트**: cgcode 자체 업데이트 소스를 확인하여 업스트림 공식판에 덮어써지지 않습니다.
  - **iOS 스타일 테마**: iOS 블루 기본 버튼과 더 큰 모서리 둥글기(라이트/다크 모두 지원).
- **v2.3 신규 기능**:
  - **전문 소프트웨어 조작**: CLI-Anything으로 GIMP, Blender, LibreOffice 등을 조작, 일괄 편집·렌더링·변환, 원본은 덮어쓰지 않음.
  - **확장 기억(선택 설치)**: claude-mem이 한 일을 기록하고 나중에 자동으로 기억, 데이터는 PC에만 저장.
  - 휴대폰 원격 제어는 LAN 개방이 필요해 보안 위험이 있어 정식판 이후로 연기.

### 기본 탑재 스킬(38개)

**자체 개발 중국어 스킬(19개)**

| 스킬 | 기능 |
|---|---|
| cg-writing | 중국어 글쓰기와 다듬기 |
| cg-meeting-notes | 회의록 정리 |
| cg-data-analysis | 데이터 분석 |
| cg-ppt | 슬라이드 제작 |
| cg-research | 자료 조사 |
| cg-code-review | 코드 리뷰 |
| cg-team | 멀티 에이전트 분업 |
| cg-daily-plan | 하루 계획 |
| cg-skill-market | 중국어 스킬 마켓: skills.sh·ClawHub·SkillHub 검색, 안전 점검 후 설치 |
| cg-web-scraper | 웹 데이터 수집, Excel로 정리 |
| cg-multi-search | 다중 엔진 검색(Baidu, Bing, Sogou 등) 통합·중복 제거 |
| cg-web-reach | 웹 통합 읽기: Bilibili, YouTube 자막, V2EX, 雪球, GitHub, RSS, 웹 페이지 |
| cg-desktop-control | PC 조작: Windows의 다른 프로그램 조작, 위험한 작업은 확인 |
| cg-long-memory | 장기 기억: 기억해 달라고 한 선호를 PC에 저장 |
| cg-offline-mode | 오프라인 모드: Ollama 설치와 GPU에 맞는 모델 선택 |
| cg-voice-studio | 음성 합성: 낭독, 방언, 목소리 복제(허락된 경우만) |
| cg-creator-studio | 크리에이터 스튜디오: 대본→콘티→음성→자막 |
| cg-pro-software | 전문 소프트웨어 조작: GIMP, Blender, LibreOffice 등(CLI-Anything) |
| cg-memory-plus | 확장 기억: claude-mem 선택 설치, 데이터는 로컬에만 |

**엄선한 오픈소스 스킬(19개)**

| 스킬 | 기능 | 출처 / 라이선스 |
|---|---|---|
| find-skills | 오픈소스 스킬 검색 및 설치 | vercel-labs/skills · MIT |
| agent-browser | 브라우저 자동화: 페이지 열기, 클릭, 입력, 스크린샷 | vercel-labs/agent-browser · Apache-2.0 |
| frontend-design | 디자인 감각 있는 웹 페이지와 UI | anthropics/skills · Apache-2.0 |
| skill-creator | 나만의 스킬 제작·테스트·개선 | anthropics/skills · Apache-2.0 |
| mcp-builder | MCP 도구 서버 개발 | anthropics/skills · Apache-2.0 |
| webapp-testing | Playwright로 웹 앱 테스트 | anthropics/skills · Apache-2.0 |
| canvas-design | 포스터와 비주얼 디자인 | anthropics/skills · Apache-2.0 |
| theme-factory | 문서·슬라이드·웹의 색상과 글꼴 테마 | anthropics/skills · Apache-2.0 |
| web-artifacts-builder | 여러 컴포넌트로 된 복잡한 웹 페이지 | anthropics/skills · Apache-2.0 |
| algorithmic-art | p5.js 제너레이티브 아트 | anthropics/skills · Apache-2.0 |
| pdf | PDF 추출·병합·분할·양식 입력·생성 | anthropics/skills · Apache-2.0 |
| internal-comms | 주간 보고, 공지 등 사내 문서 | anthropics/skills · Apache-2.0 |
| humanizer | AI 티 나는 글을 자연스럽게 | blader/humanizer · MIT |
| security-audit | 단계별 코드 보안 감사 | cloudflare/security-audit-skill · MIT |
| brainstorming | 만들기 전에 요구사항과 방안 정리 | obra/superpowers · MIT |
| writing-plans | 검증 가능한 작은 단계로 분해 | obra/superpowers · MIT |
| systematic-debugging | 근본 원인부터 수정 | obra/superpowers · MIT |
| test-driven-development | 테스트 먼저 작성 | obra/superpowers · MIT |
| verification-before-completion | 완료 선언 전 반드시 검증 | obra/superpowers · MIT |

### 스킬 안전 원칙
- 모든 기본 탑재 스킬은 라이선스를 확인하고 포함된 스크립트를 검토했습니다(키 탈취, 의심스러운 통신, 시스템 변경 없음).
- 오픈소스 스킬은 빌드 시 **고정된 커밋**으로 가져와 업스트림 변조를 방지합니다.
- 모델 안전 제한을 우회하는 탈옥 계열 스킬은 **기본 탑재하지 않으며 스킬 마켓에서도 걸러 냅니다**. 이용 약관 위반으로 키나 계정이 정지될 수 있고, 공개 배포 시 법적 위험도 있습니다.

---

## 아키텍처

### 터미널판

```
터미널
  └─ cgcode.exe (Rust)
       ├─ 중국어 마법사 / 부팅 애니메이션 / iOS 테마 / 중국어 UI
       ├─ Codex 코어(파일, 명령, 승인) ── Responses API
       └─ cgcode-chat-bridge(로컬 브리지, 127.0.0.1)
              └─ Responses ⇄ Chat Completions 변환, 속도 제한 재시도, 예비 엔드포인트
                    └─ 내 모델 API (SenseNova / DeepSeek / Qwen …)
```

| 파일 | 역할 |
|---|---|
| `codex-rs/cgcode-chat-bridge/` | Chat Completions 브리지(단위 테스트 포함) |
| `codex-rs/cli/src/cgcode.rs` | 설정, 첫 실행 마법사, 기본 스킬, 로컬 브리지 시작 |
| `codex-rs/cli/src/cgcode_intro.rs` | 터미널 부팅 애니메이션과 소리 |
| `codex-rs/tui/src/cgcode_theme.rs` | iOS 스타일 테마 7종 |
| `.github/workflows/cgcode-build.yml` | Windows 자동 빌드 및 배포 |

### 데스크톱 클라이언트

업스트림 코드를 직접 수정하지 않고 빌드 시 커스터마이징을 적용하므로 업스트림 업데이트를 따라가기 쉽습니다:

```
desktop/
├─ DSH_COMMIT              고정된 업스트림 버전
├─ apply.sh                브랜드, 중국어 기본값, 모델 설정 적용
├─ patch-bundle.mjs        기본 모델, 다중 모델, 원격 측정 끄기
├─ patch-desktop.mjs       환영 화면 키, 부팅 영상, 스킬 설치
├─ fetch-skills.mjs        고정 버전 오픈소스 스킬 가져오기 및 중국어 설명 추가
├─ upstream-skills.json    스킬 목록(출처, 버전, 중국어 설명)
└─ overlay/                아이콘, 부팅 영상, 자체 개발 중국어 스킬
ci/build-windows.sh        Windows 빌드 스크립트(Node 24 + pnpm)
```

### 빌드와 배포
- 푸시할 때마다 GitHub Actions가 Windows에서 자동 빌드합니다.
- 터미널판: 약 1.5~2시간, 성공 시 Releases에 배포.
- 데스크톱판: 약 15~25분, 설치 프로그램을 Artifact로 업로드.
- 빌드가 끝날 때마다 "빌드 성공 / 빌드 실패" Issue로 결과를 알립니다.

---

## 로드맵

### 터미널판
- [x] Chat만 지원하는 모델용 Chat Completions 브리지
- [x] SenseNova 연동, 속도 제한 재시도, 예비 엔드포인트
- [x] 중국어 첫 실행 마법사
- [x] 독립 브랜드(cgcode, `~/.cgcode`), ChatGPT 로그인과 원격 측정 제거
- [x] 완전 중국어 UI
- [x] iOS 스타일 테마 7종 + `/theme`
- [x] 공식 부팅 영상(터미널 문자 아트판) + 부팅 음악
- [x] 중국어 스킬 6개 기본 탑재
- [x] Windows 자동 빌드 및 배포
- [ ] 실제 기기 검수
- [ ] 검수 피드백 반영, 안정판 v1.0
- [ ] 실행 로그 기능

### 데스크톱 클라이언트
- [x] cgcode 브랜드, 기본 중국어 UI
- [x] 다중 모델: SenseNova(기본), Qwen, Kimi
- [x] 환영 화면 키 입력, 진입 장벽 제로
- [x] 세션 원격 측정과 제품 분석 끄기
- [x] 원본 화질 부팅 영상
- [x] 멀티 에이전트 협업, Office, 예약 작업
- [x] 스킬 38개 기본 탑재 + 중국어 스킬 마켓
- [x] v2.1: 로컬 음성 입력, 기록 검색, PC 조작, 웹 통합 읽기, 장기 기억, 안심 모드
- [x] v2.2: 로컬 오프라인 모델, 음성 합성, 크리에이터 스튜디오, 통계 바
- [x] v2.3: 전문 소프트웨어 조작, 확장 기억(선택)
- [ ] 휴대폰 원격 제어(정식판 이후)
- [ ] 실제 기기 검수
- [ ] 업스트림 플러그인 마켓을 cgcode 추천으로 교체
- [ ] Claude 스타일 UI 다듬기(반투명 유리 등 iOS 효과 완전 구현)
- [x] 플러그인 "발견" 탭 기본 활성화; 스킬 검색과 원클릭 설치는 스킬 마켓 스킬로 제공
- [x] 자동 업데이트: cgcode 자체 업데이트 소스(GitHub Pages) 사용
- [x] 정식판 1.0.0: iOS 블루 기본 버튼, 더 큰 모서리 둥글기
- [ ] 코드 서명
- [x] 정식판 1.0.0 설치 프로그램(현재 미서명)

---

## 알려진 제한 사항

- 아직 서명되지 않아 첫 실행 시 Windows SmartScreen 경고가 표시됩니다. "실행"을 선택하세요.
- 두 버전 모두 실제 Windows PC에서의 검수가 완료되지 않았습니다.
- 데스크톱판은 현재 GitHub에 로그인해 Actions에서 내려받아야 합니다. 정식판은 Releases에 배포할 예정입니다.
- 일부 모델 서비스(SenseNova 등)는 속도 제한이 있어 혼잡 시간에는 자동 재시도에 시간이 조금 걸릴 수 있습니다.

---

## 자주 묻는 질문

**SenseNova를 꼭 써야 하나요?**
아니요. OpenAI 호환 엔드포인트라면 무엇이든 됩니다. 마법사나 설정에서 엔드포인트, 키, 모델명을 바꾸세요.

**부팅 애니메이션이 길어요 / 소리를 끄고 싶어요.**
터미널판은 `cgcode.toml`의 `[intro]`에서 `enabled` 또는 `sound`를 `false`로 바꾸세요. 데스크톱판은 클릭하거나 아무 키나 눌러 건너뜁니다.

**코드나 대화가 업로드되나요?**
아니요. 대화는 내가 설정한 모델 서비스로만 전송되며, 원격 측정은 기본적으로 모두 꺼져 있습니다.

**스킬을 더 설치하려면?**
데스크톱판에서 "○○를 할 수 있는 스킬 찾아 줘"라고 말하면 스킬 마켓을 검색해 중국어로 보여 주고, 안전 점검 후 설치합니다.

---

## 라이선스와 감사의 말

- 터미널판은 [OpenAI Codex](https://github.com/openai/codex)(Apache License 2.0)를 기반으로 하며 원본 `LICENSE`와 `NOTICE`를 유지합니다.
- 데스크톱 클라이언트는 DeepSeek Harness(MIT License)를 기반으로 구축되었습니다. cgcode는 독립 제품이며 DeepSeek, OpenAI와 제휴 관계가 없습니다.
- 기본 탑재 오픈소스 스킬은 [vercel-labs/skills](https://github.com/vercel-labs/skills)(MIT), [vercel-labs/agent-browser](https://github.com/vercel-labs/agent-browser)(Apache-2.0), [anthropics/skills](https://github.com/anthropics/skills)(Apache-2.0)에서 가져왔으며 각 스킬 폴더에 원래 라이선스를 유지합니다.
