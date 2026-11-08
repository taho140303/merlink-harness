"""Render the Merlink execution plan from the reviewed in-repository planning model.
Only writes files inside this directory. Does not execute application code or migrations.
"""
from __future__ import annotations
import json, re, runpy, itertools, collections
from pathlib import Path

HERE=Path(__file__).resolve().parent
repo=HERE.parent.parent
m=runpy.run_path(str(HERE/'plan_model.py'))
S=m['S']; R=m['R']; BASE=m['BASE']; SOURCE_BASE=m.get('SOURCE_BASE',BASE); EXEC_BASE=m.get('EXEC_BASE',BASE); PHASES=m['PHASES']; AUTH=m['AUTH']
byid={s['id']:s for s in S}; req={r[0]:r for r in R}
assert len(byid)==len(S) and len(req)==len(R)
for s in S:
    assert re.fullmatch(r'phase[1-9]\d*_sprint[1-9]\d*_deep\d+',s['id'])
    assert all(d in byid and d!=s['id'] for d in s['deps'])
    assert all(x in req and req[x][4]==s['id'] for x in s['reqs'])
    for p,a,b,focus in s['refs']:
        source=repo/p
        assert source.is_file(), (s['id'],p)
        assert 1<=a<=b<=len(source.read_text(encoding='utf-8').splitlines()),(s['id'],p,a,b)
    # Shared resource locks for test artifacts are isolated per worktree and sprint.
    s['locks']=[x+'::'+s['id'] if x in ('sqlite-testdb','fake-provider','visual-viewport','provider-fixture','app-port') else x for x in s['locks']]

def depth(s):
    return 0 if not any(byid[d]['phase']==s['phase'] for d in s['deps']) else 1+max(byid[d]['deep'] for d in s['deps'] if byid[d]['phase']==s['phase'])
for s in S: assert depth(s)==s['deep'],(s['id'],depth(s),s['deep'])

def walk(node,stack=()):
    assert node not in stack,('cycle',stack,node)
    for d in byid[node]['deps']: walk(d,stack+(node,))
for s in S: walk(s['id'])

def gate(s): return 'gate_'+s['id']
def filename(s): return s['id']+'.md'
def dep_kind(s,d):
    t=byid[d]['title'].lower()
    if 'migration' in t: return 'MIGRATION_DEPENDENCY'
    if 'contract' in t or 'artifact' in t: return 'DATA_CONTRACT_DEPENDENCY'
    if 'dto' in t or 'domain types' in t: return 'TYPE_DEPENDENCY'
    if 'fixture' in t: return 'TEST_FIXTURE_DEPENDENCY'
    if 'bootstrap' in t: return 'BUILD_DEPENDENCY'
    return 'INTEGRATION_DEPENDENCY' if byid[d]['phase']!=s['phase'] else 'RUNTIME_DEPENDENCY'

def conflict(a,b):
    same=set(a['paths'])&set(b['paths'])
    shared=set(a['locks'])&set(b['locks'])
    if same:return ('C1',','.join(sorted(same)))
    if shared:return ('C4/C8',','.join(sorted(shared)))
    return ('NONE','Disjoint exact write paths; shared types frozen by predecessors; worktree-local test resources')

def test_command(s):
    if s['phase']==1:return 'python3 docs/plan/validate_plan.py --contract-evidence '+s['paths'][0]+'  # future target after authority/evidence file exists'
    if s['phase']==5:return 'pnpm test && pnpm typecheck && pnpm lint && pnpm build  # future target, after dependencies exist'
    if s['phase']==6:return 'pnpm test && pnpm typecheck && pnpm lint && pnpm build && cargo test --manifest-path src-tauri/Cargo.toml  # future target'
    tests=[p for p in s['paths'] if p.startswith('src-tauri/tests/')]
    return 'cargo test --manifest-path src-tauri/Cargo.toml '+('--test '+Path(tests[0]).stem if tests else '')+'  # future target, after bootstrap'

interfaces={
'phase1_sprint1_deep0':'CONTRACT_EVIDENCE {issue_id, old_authority, proposed_disposition, owner_approval_sha, affected_sprints}; no implementation interface.',
'phase1_sprint2_deep1':'MIGRATION_DECISION {baseline_manifest_sha, version_path, backup_policy, authorized_revision}; absent approval = CONTRACT_BLOCKED.',
'phase1_sprint3_deep0':'ARTIFACT_CONSUMER_FIXTURE {WorldProbeReportV1, WorldWriteInputV1, exact_input_hash, probe_refs, approved_revision}.',
'phase2_sprint1_deep0':'EXISTING_INTERFACE: React Router build; PROPOSED_INTERFACE: minimal Tauri Builder and crate modules, no registered business command yet.',
'phase2_sprint2_deep1':'PROPOSED_INTERFACE: BusinessFlowData, TrackedField<T>, ProvenanceSource, canonical_state(&State)->Result<Vec<u8>>, CodeAllocator::allocate.',
'phase2_sprint3_deep1':'PROPOSED_INTERFACE: DbManager::initialize_and_migrate(app_data_dir)->Result<DbPool,AppError>; verify_schema_and_integrity(conn).',
'phase2_sprint4_deep3':'PREREQUISITE_INTERFACE: frozen IpcError/DTO. PROPOSED_INTERFACE: LlmProvider::{chat_with_tools,stream_completion}, CredentialManager, Redactor.',
'phase2_sprint5_deep2':'PROPOSED_INTERFACE: 40 request/response DTOs, ChatStreamEvent/WorkflowProgressEvent/SceneGenerationStreamEvent, IpcError {code,message,retryable}; generated TS binding.',
'phase2_sprint6_deep2':'PROPOSED_INTERFACE: DraftRepo, CatalogRepo, WorkflowReadRepo transaction-scoped query traits; TempStore does not prove completion.',
'phase3_sprint1_deep0':'PROPOSED_INTERFACE: plan_outline_runs, split_balanced, plan_phase, context_range, ProgressSummary(confirmed,ai_suggested,denominator=46), validate_draft_for_commit.',
'phase3_sprint2_deep0':'PROPOSED_INTERFACE: DraftService::{seal_version,restore_version,get_state}; guarded DraftRuntimeEntry; draft/version IPC #5–8,#10–11.',
'phase3_sprint3_deep0':'PROPOSED_INTERFACE: WorldProbeReportV1 {schemaVersion,codecVersion,assignedScope,reportText}; WorldWriteInputV1 {schemaVersion,codecVersion,assignedScope,consumedInput,probeRefs}.',
'phase3_sprint4_deep1':'PROPOSED_INTERFACE: ToolRegistry::lookup(meta), read_tools::{get_field_null,get_array_element_fields,get_state,search_*}; no direct mutation.',
'phase3_sprint5_deep2':'PROPOSED_INTERFACE: ToolExecutor::execute(call,guard)->ToolOutcome {status,results,authenticated_effect}; CascadePreview; writes stay in WorkingState clone.',
'phase3_sprint6_deep3':'PROPOSED_INTERFACE: PatchService::{persist_proposal,confirm,reject,confirm_all}; TX08 and TX02; patch IPC #12–15.',
'phase3_sprint7_deep4':'PROPOSED_INTERFACE: ChatService::{send,cancel}; PromptComposer::compose; chat IPC #1–4,#9 and ChatStreamEvent.',
'phase3_sprint8_deep1':'PROPOSED_INTERFACE: canonical_diff(S0,Sn)->PatchV2; replay(S0,PatchV2)->State; plan_digest, request_digest, codecVersion.',
'phase3_sprint9_deep2':'PROPOSED_INTERFACE: verify_historical_round(read_tx,id,codecs,head)->{integrity,authorization_attestation,applicability}.',
'phase3_sprint10_deep4':'PROPOSED_INTERFACE: seal_or_retry_round(guard,tx,authorized_plan,effects)->Historical|Sealed|Conflict|StaleHead|RestoreDiverged.',
'phase4_sprint1_deep0':'PROPOSED_INTERFACE: WorkflowService::{start_or_resume,cancel,status}; workflow IPC #16–18 and WorkflowProgressEvent.',
'phase4_sprint2_deep0':'PROPOSED_INTERFACE: CommitService::{preview,commit,reopen}; commit IPC #19–21; TX05 one atomic transition.',
'phase4_sprint3_deep1':'PROPOSED_INTERFACE: ProjectService and CharacterService, IPC #22–28, stable read models.',
'phase4_sprint4_deep1':'PROPOSED_INTERFACE: WritingPersistence::{get_chapter,set_scene_lock,set_bulk_lock,save_chapter,get_receipt}; IPC #29–31,#35–37.',
'phase4_sprint5_deep2':'PROPOSED_INTERFACE: WritingGeneration::{start,cancel,expand_scope}; IPC #32–34, SceneGenerationStreamEvent.',
'phase5_sprint1_deep0':'PROPOSED_INTERFACE: typed desktop invoke/channel adapter; SSR-safe failure; generated DTOs remain read-only.',
'phase6_sprint2_deep0':'PROPOSED_INTERFACE: Tauri command registration for exactly 40 names and 3 event channels; startup recovery before serving IPC.',
}

def sprint_md(s):
    i=s['id']; p=s['phase']; ds=s['deps']; peers=[x for x in S if x['phase']==p and x['deep']==s['deep'] and x['id']!=i]
    lines=[f'---\nsprint_id: {i}\ntitle: {s["title"]}\nphase: {p}\nsprint: {s["sprint"]}\ndeep: {s["deep"]}\nbaseline_commit: {BASE}\nstatus: PLANNED\n---',f'# {i} — {s["title"]}',
    '## 1. Sprint identity',f'Engineering phase {p} ({PHASES[p]}), within-phase depth {s["deep"]}; baseline is the Git commit above, not an uncommitted workspace. **PROPOSED, NOT IMPLEMENTED.**',
    '## 2. Objective',s['title']+'. Observable handoff: '+'; '.join(s['handoff'])+'.',
    '## 3. Scope boundaries','**IN_SCOPE:** '+ '; '.join(s['steps'])+'\n\n**OUT_OF_SCOPE / MUST NOT:** modify authoritative `docs/business/**`, `docs/db.dbml`, `docs/architecture/decisions/**`; use real application data; run production migration; add client request IDs to SF-06; invent business semantics; rewrite another sprint’s files.',
    '## 4. Authoritative contract snapshot']
    lines += ['- '+x for x in s['contract']]
    excerpt_count=0
    for rid in s['reqs']:
        r=req[rid]; lines.append(f'- **{rid}** `{r[1]}#L{r[2].replace("-","-L")}`: {r[3]} (planning coverage `{r[5]}`).')
        # Bounded verbatim context avoids forcing a subagent to infer critical qualifiers.
        if not rid.startswith(('REQ-CMD-','REQ-TX-','REQ-SPECTEST-')) and excerpt_count<8:
            src_path=repo/r[1]; a,b=map(int,r[2].split('-')); rows=src_path.read_text(encoding='utf-8').splitlines()[a-1:b]
            picks=[]
            for off,line in enumerate(rows):
                text=line.strip()
                if text and not text.startswith('|---') and not text.startswith(':---'):
                    picks.append(f'L{a+off}: {text[:300]}'.rstrip())
                if len(picks)>=4:break
            if picks:
                lines.append('  Normative excerpt (verbatim baseline lines, truncated to 300 chars; use `sed` for the full range):')
                lines.extend('  > '+x.replace('|','/') for x in picks)
                excerpt_count+=1
    lines.append('**Targeted source references (read only these ranges unless a test proves broader context is necessary):**')
    for src,a,b,focus in s['refs']:
        lines.append(f'- `{src}#L{a}-L{b}` — focus: {focus}; CLI: `sed -n "{a},{b}p" {src}` (or `rg -n "<symbol>" {src}` for a changed anchor).')
    ownedcmd=[int(x.rsplit('-',1)[1]) for x in s['reqs'] if re.fullmatch(r'REQ-CMD-\d{3}',x)]
    if ownedcmd:
        be=(repo/'docs/business/be-core.md').read_text(encoding='utf-8').splitlines()
        lines.append('**Owned IPC command contracts (verbatim narrow BE §10.2 rows; proposed, not currently implemented):**')
        for n in ownedcmd:
            name=m['COMMANDS'][n-1]
            found=[(j+1,row) for j,row in enumerate(be) if row.startswith('| **`'+name+'`** |')]
            assert len(found)==1,(i,name,found)
            j,row=found[0];lines.append(f'- IPC-{n:02d} `{name}` — `docs/business/be-core.md#L{j}`: '+row.replace('|',' · ').strip(' ·'))
    lines += ['## 5. Prerequisites',f'`depends_on: {json.dumps(ds,ensure_ascii=False)}`.']
    if ds:
        for d in ds:
            x=byid[d];lines.append(f'- `{d}` [{dep_kind(s,d)}] produces {x["handoff"][0]}; inspect its accepted commit, exported artifact/test and `{gate(x)}`. If absent, do not start.')
    else: lines.append('- None within the implementation DAG; first verify baseline SHA, clean/isolated worktree, platform tools and owner/ADR status.')
    lines += ['## 6. Inputs','Required: accepted outputs of all predecessors; frozen committed SSOT ranges above; actual source paths listed below. Optional context: existing UI mocks for fixtures only. No `report/` input; no historical research.']
    gen=['src/shared/backend/generated.ts'] if i=='phase2_sprint5_deep2' else []
    lines += ['## 7. File ownership','```yaml','allowed_write_paths:']+['  - '+x for x in s['paths']]+['forbidden_write_paths:','  - docs/business/**','  - docs/db.dbml','  - docs/architecture/decisions/**','  - app-data/**','  - report/**','shared_read_only_paths:']+['  - '+x for x in dict.fromkeys(r[0] for r in s['refs']) if x not in s['paths']]+(['generated_paths:']+['  - '+x+'  # generated by frozen ts-rs generator only; never hand-edited' for x in gen] if gen else ['generated_paths: []'])+['```']
    if i=='phase2_sprint5_deep2':lines.append('`src/shared/backend/generated.ts` is a generated write target: it may be produced only by the frozen ts-rs generator, never hand-edited. It appears in allowed_write_paths solely so the gate can authorize generator output.')
    if i=='phase2_sprint1_deep0':lines.append('The bootstrap may create only empty compile-only placeholders for the generated stub paths above; each belongs to its future sprint owner, contains no business behavior, and must be replaced there. Bootstrap may not fill any other sprint’s implementation.')
    lines += ['Any allowed path may be absent at baseline (not only `src-tauri/`; new FE adapter/shim files are also future targets). Verify with `test -e`/`git ls-files` before editing; existing files are read first. Any extra path requires central integration approval, not a silent scope expansion.']
    lines += ['## 8. Public interfaces',interfaces.get(i,'PROPOSED_INTERFACE: '+'; '.join(s['handoff'])+'.')+' Every consumed prerequisite interface is `PREREQUISITE_INTERFACE`, not an existing runtime guarantee. If a named signature conflicts with committed SSOT, stop `CONTRACT_BLOCKED`.']
    lines += ['## 9. Implementation requirements']+['- '+x for x in s['steps']]
    lines += ['- Prefer official compatible CLI/tooling over manual generated files; verify installed version and diff. No external LLM/Jev request without authorization; optional Jev advice never overrides SSOT.']
    lines += ['## 10. Domain invariants']+['- '+x for x in s['contract']]+['- No partial state escapes; protected provenance, explicit authorization, no external-effect recovery replay where applicable.']
    lines += ['## 11. Failure matrix','| Trigger | Expected behavior / classification | Durable effects and recovery | Required test |','|---|---|---|---|']
    for n,x in enumerate(s['failures'],1):
        trigger,_,rest=x.partition('=>');lines.append(f'| {trigger.strip()} | {rest.strip()} | No unauthorized partial commit; re-read actual durable state before retry | {i}-F{n} |')
    lines.append(f'| Missing accepted predecessor / baseline mismatch | `CONTRACT_BLOCKED`; do not run implementation | No user-data effects; record issue and owner in sprint evidence | {i}-F0 |')
    lines += ['## 12. Acceptance tests','Tests below name **future targets**, not tests executed during planning. Each row must become an executable test before the sprint can be accepted.','| Test ID | Initial state | Input / action | Expected output and state | Forbidden side effects / invariant |','|---|---|---|---|---|']
    for x in s['cases']:
        tid,_,desc=x.partition(':')
        if s['phase']==1: initial='accepted baseline + owner evidence worktree; no prior sprint acceptance'
        elif s['phase']==5: initial='accepted backend gate + fake typed Tauri transport fixture'
        elif s['phase']==6: initial='accepted integrated worktree or isolated CI checkout'
        elif s['id']=='phase2_sprint1_deep0': initial='clean scaffold worktree with no src-tauri and no application data'
        elif any(w in ' '.join(s['paths']).lower() for w in ('storage/','migration','service/','sqlite')): initial='disposable uniquely-named SQLite S0 + accepted predecessor fixture'
        else: initial='pure Rust fixture (no DB/network/Tauri) + accepted predecessor types'
        scenario=desc.strip().split('=>')[0].strip()
        lines.append(f'| {tid} | {initial} | In the owned test target, set up the fixture and run the scenario: {scenario} | {desc.strip()} | No unrelated write; preserve exact expected state and listed invariant |')
    for n,x in enumerate(s['failures'],1):
        trigger,_,rest=x.partition('=>')
        lines.append(f'| {i}-F{n} | accepted prerequisite fixture / isolated worktree | Inject {trigger.strip()} into owned operation | {rest.strip()} | No unauthorized commit or external replay |')
    for n in ownedcmd:
        name=m['COMMANDS'][n-1]; be=(repo/'docs/business/be-core.md').read_text(encoding='utf-8').splitlines(); detail=next(x for x in be if x.startswith('| **`'+name+'`** |'))
        lines.append(f'| IPC-CMD-{n:03d} | isolated authorized and unauthorized caller fixtures | Invoke `{name}` with valid request and invalid/stale variant; use frozen DTO | {detail.replace("|"," / ").strip()} | No unlisted command, no unauthorized write; assert event/error/transaction result |')
    for rid in s['reqs']:
        if rid.startswith('REQ-TX-'):
            n=int(rid[-3:]); detail=(repo/'docs/business/be-core.md').read_text(encoding='utf-8').splitlines()[498+n-1]
            lines.append(f'| TX-{n:03d} | disposable S0 + failure injection at every write step | Execute TX-{n:02d} then inject error/lose response | {detail.replace("|"," / ").strip()} | Atomic rollback, no unrelated durable effect; retry checks receipt |')
        if rid.startswith('REQ-SPECTEST-'):
            testid=rid.removeprefix('REQ-SPECTEST-'); line=int(req[rid][2].split('-')[0]); detail=(repo/'docs/business/be-core.md').read_text(encoding='utf-8').splitlines()[line-1]
            cells=[c.strip() for c in detail.split('|') if c.strip()]
            if len(cells)>=3: lines.append(f'| {testid} | disposable/synthetic fixture described by BE | {cells[-2].replace("|","/")} | {cells[-1].replace("|","/")} | Only owner-authorized effects; old v1-only statements interpreted as legacy, no B+ regression |')
    lines.append(f'| {i}-F0 | predecessor gate absent or baseline SHA drift | Attempt dispatch before predecessor accepted | CONTRACT_BLOCKED; no implementation started | No source/DB modification |')
    if s['phase']==1:
        lines.append('Contract evidence must include `CONTRACT_STATUS: ACCEPTED` (or `INVENTORY_ACCEPTED` only for inventory; descendants still blocked on affected issues), `AUTHORITY_SHA`, `ISSUE_ID`, `SOURCE_ANCHOR`, and `DISPOSITION`; the validator checks format, but a human/owner must verify the approval SHA. A ledger alone cannot unblock SQL, artifact codec or changed product policy.')
    if s['phase'] in (2,3,4) and i!='phase2_sprint1_deep0':
        lines.append('Backend module wiring rule: this sprint must not edit any `mod.rs`/`lib.rs`. Its integration test loads each owned implementation file via `#[path = "..."] mod <name>;` and fixture modules via the same mechanism. Production registration occurs only in P6S2 after this sprint is accepted. A compile success is therefore scoped to this test target, not a claim that the whole crate is wired.')
    fe=any(p.startswith('src/') and not p.startswith('src-tauri/') for p in s['paths']) or s['id']=='phase6_sprint3_deep1'
    suffix=' For FE changes, visual comparison at 1280/1279px and each declared viewport is mandatory; if tooling is unavailable the sprint is `BLOCKED_VISUAL_VERIFICATION` and cannot be accepted. `VISUAL_NOT_VERIFIED` is not a waiver.' if fe else ''
    lines.append(f'Future command after the target files exist: `{test_command(s)}`.'+suffix)
    lines += ['## 13. Database and migration safety',('No persistent database in this sprint; fixture DB (if any) is disposable and uniquely named per worktree.' if not any(w in ' '.join(s['paths']) for w in ('storage/','service/','migrations/')) else 'Use `BEGIN IMMEDIATE` only for the assigned TX; WAL/FULL/FK ON/busy_timeout as specified. Use unique disposable DB/temp root per worktree. Never open a user application DB; no destructive migration without separate authorization. Backup/rollback and PRAGMA user_version are owned by P2S3. TX04 never rolls back committed TX08.'),
    '## 14. Concurrency contract']
    if peers:
        for x in peers:
            c,why=conflict(s,x);lines.append(f'- Peer `{x["id"]}`: {"NOT PARALLEL" if c!="NONE" else "candidate parallel only after frozen contracts/gates"}; {c}: {why}.')
    else: lines.append('- No same-phase/same-depth peer. Cross-phase scheduling still requires exact predecessor gates and conflict scan.')
    lines.append('Isolation: own worktree/branch from baseline or accepted integration SHA; unique temp root, test DB and port; no shared mutable process/global fixture. Same depth alone is NOT permission to dispatch.')
    lines += ['## 15. Deliverables']+['- [ ] '+x for x in s['handoff']]+['- [ ] Tests above and evidence including command exit codes, scope diff and accepted commit SHA.']
    lines += ['## 16. Completion criteria',f'`SPRINT_IMPLEMENTED` requires artifacts, `SPRINT_TESTED` requires executed acceptance tests, `SPRINT_INTEGRATED` requires merged checks, and **`SPRINT_ACCEPTED` requires `{gate(s)}` accepted**. A contract-evidence sprint cannot be ACCEPTED for an affected implementation successor while its owner decision is unresolved. No agent declaration alone unlocks successors. All required assertions, file scope and baseline must pass.']
    if i=='phase1_sprint1_deep0':
        lines.append('**Contract issue ledger (open):** '+ '; '.join(f'{x[0]} = {x[1]} → {", ".join(x[2])}' for x in m['CONTRACT_ISSUES'])+'. This sprint may be gate-accepted as `INVENTORY_ACCEPTED` after the ledger and owner-escalation map exist; that acceptance does **not** clear any issue. Each listed successor also carries the issue IDs in its manifest `contract_blockers` and must stop until the owner decision is recorded.')
    lines += ['## 17. Handoff contract']+['- '+x for x in s['handoff']]+[f'- Exact artifacts: {", ".join(s["paths"])}; accepted integration SHA and `{gate(s)}` record. A successor must verify exports/ABI rather than infer from prose.']
    lines += ['## 18. Resume and failure behavior','Not started: verify baseline and deps. Partial/test failure: inspect git status, changed paths and durable test DB, resume only safe incomplete steps. Implemented but unintegrated: do not dispatch successors. Interrupted agent/rejected patch: preserve branch, rebase only after review, rerun tests. Specification conflict: `CONTRACT_BLOCKED`, record evidence, request authority resolution; never edit SSOT in this sprint.']
    if s['blocked']:lines.append('**Explicit unblock requirements:** '+', '.join(s['blocked'])+'.')
    lines += ['\n---','Subagent budget: 256K tokens maximum; start with the bounded references above, do not read entire SSOT by default. CLI-first with verified tool compatibility. Treat Jev/TypeSafe as optional advisory only when available and authorized; never send private repository content externally without consent.']
    return '\n\n'.join(lines)+'\n'

for s in S:(HERE/filename(s)).write_text(sprint_md(s),encoding='utf-8')

G=[dict(id=gate(s),sprint_id=s['id'],phase=s['phase'],depth=s['deep'],status='PLANNED',requires=[gate(byid[d]) for d in s['deps']],checks=(['owned artifacts exist','planned scope only; generated output allowed only for ts-rs'] if s['id']=='phase2_sprint5_deep2' else ['owned artifacts exist','allowed_write_paths only'])+['required tests executed and pass','frozen ABI/schema match','build/typecheck and predecessor regression pass','integrated revision SHA recorded','contract evidence recorded as INVENTORY_ACCEPTED; issue ledger remains open' if s['id']=='phase1_sprint1_deep0' else 'no unresolved contract blocker']) for s in S]
def section(src,ran):
    line=int(ran.split('-')[0])
    if src.endswith('be-core.md'):
        return 'BE §'+str(next((n for end,n in [(255,1),(359,2),(509,3),(737,4),(900,5),(1581,6),(1680,7),(2026,8),(2099,9),(2414,10),(2514,11),(2612,12),(2684,13),(99999,14)] if line<=end),0))
    if src.endswith('core.md'):return 'Core §'+('1.3' if line<111 else '1.6' if line<339 else '3.9' if line>=835 else '3.6' if line>=662 else '3.1' if line>=596 else '2')
    if src.endswith('fe-core.md'):return 'FE §'+('12.1' if 495<=line<=514 else '9' if 294<=line<=440 else '8' if 267<=line<=292 else '1–7')
    if src.endswith('db.dbml'):return 'DBML tables / physical constraints'
    return 'Owner decision / supporting contract'
INV=[dict(requirement_id=rid,source_file=src,source_section=section(src,ran),source_anchor=f'{src}#L{ran.replace("-","-L")}',normative_statement=norm,implementation_owner_sprint=owner,verification_owner=gate(byid[owner]),status=status) for rid,src,ran,norm,owner,status in R]
manifest=dict(schema_version=1,project='merlink-harness',baseline_commit=BASE,source_baseline_commit=SOURCE_BASE,execution_baseline_commit=EXEC_BASE,planning_status='PLANNED',authority_files=AUTH,baseline_status='clean committed frontend-only; no Rust/Tauri/migrations/CI at HEAD',phases=[{'phase':k,'name':v,'meaning':'engineering capability, not business-flow Phase 0–6'} for k,v in PHASES.items()],sprints=[dict(sprint_id=s['id'],filename=filename(s),title=s['title'],phase=s['phase'],depth=s['deep'],depends_on=[dict(sprint_id=d,type=dep_kind(s,d),reason=byid[d]['handoff'][0]) for d in s['deps']],required_integration_gates=[gate(byid[d]) for d in s['deps']],acceptance_gate=gate(s),acceptance_mode=('INVENTORY_ACCEPTED_ALLOWED' if s['id']=='phase1_sprint1_deep0' else 'STANDARD'),ownership_mode=('bootstrap_then_handoff' if s['id']=='phase2_sprint1_deep0' else 'final_composer' if s['id']=='phase6_sprint2_deep0' else 'exclusive'),allowed_write_paths=s['paths'],generated_write_paths=(['src/shared/backend/generated.ts'] if s['id']=='phase2_sprint5_deep2' else []),forbidden_write_paths=['docs/business/**','docs/db.dbml','docs/architecture/decisions/**','report/**','app-data/**'],shared_resource_locks=s['locks'],parallel_eligible_with=[x['id'] for x in S if x['phase']==s['phase'] and x['deep']==s['deep'] and x['id']!=s['id'] and conflict(s,x)[0]=='NONE'],contract_blockers=s['blocked'],contract_issue_ids=[i[0] for i in m['CONTRACT_ISSUES'] if s['id'] in i[2]],requirement_ids=s['reqs'],acceptance_status='PLANNED') for s in S],integration_gates=G,requirements=INV,contract_issues=[dict(id=i[0],description=i[1],affected_sprints=i[2],status='OPEN') for i in m['CONTRACT_ISSUES']])
(HERE/'manifest.json').write_text(json.dumps(manifest,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')

# Coverage matrix: one owner/test/gate per mandatory requirement; contract blockers stay visible.
cv=['# Mandatory requirements and bidirectional coverage','',f'Baseline `{BASE}`; {len(R)} grouped mandatory obligations. Grouped rows preserve qualifiers in their narrow source snapshots; they are not implementation PASS.','', '| Requirement | Authority | Source section | Source anchor | Normative contract | Implementing sprint | Acceptance test | Verification owner/gate | Planning status |','|---|---|---|---|---|---|---|---|---|']
for rid,src,ran,norm,owner,status in R:
    assert owner in byid
    s=byid[owner]; test=('IPC-CMD-'+rid[-3:] if re.fullmatch(r'REQ-CMD-\d{3}',rid) else 'TX-'+rid[-3:] if rid.startswith('REQ-TX-') else s['cases'][0].split(':')[0])
    cv.append(f'| {rid} | `{src}` | {section(src,ran)} | `#L{ran.replace("-","-L")}` | {norm} | [{owner}](<{filename(s)}>) | {test} | {gate(s)} | {status} |')
cv+=['','## Inverse ownership audit','']
for s in S:
    owned=[r[0] for r in R if r[4]==s['id']]
    cv.append(f'- `{s["id"]}`: '+(', '.join(owned) if owned else 'enabling infrastructure / contract prerequisite: '+s['title']+'. Source justification in its sprint contract.') )
blocked=[r[0] for r in R if r[5]=='CONTRACT_VALIDATION_REQUIRED']
cv+=['','## Coverage calculation',f'- Total mandatory grouped obligations: {len(R)}.',f'- Assigned implementation owners: {len(R)}.',f'- Contract-validation required: {len(blocked)} ({", ".join(blocked)}).','- Unresolved without any owner: 0; this does **not** mean the contract blockers are resolved.','- Requirements without named acceptance test/gate: 0.','- Implementation tests executed during planning: 0.','- Spec test matrices T-*, MIG/WF/IPC, WF-C/DB-P, REC and the 40-command inventory are allocated to sprint acceptance; final full-matrix execution belongs P6S3.','- If a contract prerequisite remains unresolved, successor status is CONTRACT_BLOCKED, not READY; external owner authority is required before implementation.']
(HERE/'coverage-matrix.md').write_text('\n'.join(cv)+'\n',encoding='utf-8')

pairs=[]
for p in PHASES:
    for dep in sorted(set(x['deep'] for x in S if x['phase']==p)):
        group=[x for x in S if x['phase']==p and x['deep']==dep]
        for a,b in itertools.combinations(group,2):pairs.append((a,b,*conflict(a,b)))
# Critical path across all nodes, not a duration estimate.
cache={}
def chain(i):
    if i not in cache:
        options=[chain(d) for d in byid[i]['deps']]
        cache[i]=max(options,key=len,default=[])+[i]
    return cache[i]
critical=max((chain(s['id']) for s in S),key=len)
graph=['# Dependency DAG and concurrency proof','',f'Baseline `{BASE}`. An engineering phase is NOT a user-facing business-flow phase. Depth is computed from in-phase predecessors only; cross-phase edges remain explicit in `manifest.json`.','', '## Phase overview']
for p,name in PHASES.items():
    graph.append(f'- Phase {p}: {name}. '+'; '.join(f'deep{d}: '+', '.join(x['id'] for x in S if x['phase']==p and x['deep']==d) for d in sorted(set(x['deep'] for x in S if x['phase']==p))))
graph+=['','## High-level graph','```mermaid','flowchart LR']
for p,name in PHASES.items():graph.append(f'  P{p}["P{p} {name}"]')
for p in range(1,6):graph.append(f'  P{p} --> P{p+1}')
graph+=['```','The arrows above are orientation only: actual executable prerequisites are the following edges, including cross-phase parallelism; no implicit all-of-previous-phase barrier.','', '## Exact sprint edges','```mermaid','flowchart LR']
for s in S:
    graph.append(f'  {s["id"]}["{s["id"]}"]')
    for d in s['deps']:graph.append(f'  {d} --> {s["id"]}')
graph+=['```','', '## Integration gates and READY rule','Each sprint has one explicit acceptance gate `gate_<sprint_id>`; this is an integration gate for its phase/depth. Independent siblings do not hold each other hostage. A successor requires only the accepted gates of its actual predecessors, not a blanket phase barrier. Each gate checks scope, artifacts, tests, ABI/schema, merged build, regressions, blockers and SHA.','', '## Same-phase / same-depth conflict matrix (ALL pairs)','| Pair | C1..C10 check | Status | Concrete reason |','|---|---|---|---|']
for a,b,c,why in pairs:graph.append(f'| {a["id"]} ↔ {b["id"]} | {c} | {"SERIALIZE" if c!="NONE" else "CANDIDATE ONLY"} | {why}; C2/C3/C5/C6/C7/C9/C10 frozen predecessor ownership and gate review |')
graph+=['','Same depth is insufficient: check active sprint paths/locks across **all** phases before dispatch. A test DB/port is worktree-unique. Shared Cargo/lockfile, module registration, global router and i18n have one owner at a time.','', '## Critical path (no duration estimate)',' → '.join('`'+x+'`' for x in critical),f'Length: {len(critical)} sprint nodes. Contract blockers may halt this path; no ETA is inferred.','', '## Shared-contract ownership','- P2S1 owns Cargo/Tauri/module shells; P2S5 freezes DTO/TS generated binding; P6S2 alone composes global lib/ipc mod/router/i18n after acceptance.','- P2S3 owns all migrations/schema version; no parallel SQL writers. P3S2 owns draft repo service, P3S6 pending repo, P3S10 workflow seal repo, P4S2 commit repo, P4S4 writing repo.','- P5S5 alone owns ChapterDetailView and scenePreview; other FE sprints must not touch them.','- P1S1/P1S2/P1S3 own separate contract evidence files; unresolved decisions must be escalated, not silently selected.']
(HERE/'dependency-graph.md').write_text('\n'.join(graph)+'\n',encoding='utf-8')

readme=f'''# Merlink Harness autonomous implementation plan

Source baseline commit: `{SOURCE_BASE}` on `main` (execution/plan commit: `{EXEC_BASE}`); working tree was clean when authored. This is a **plan only**. At baseline there is a React Router 8/React 19 mock frontend and 39 Node tests, but no Rust, Tauri, SQLite migrations, backend tests or CI. No implementation or migration was run to author these files.

## Authority and navigation
Owner decisions in `docs/architecture/decisions/sf06-owner-ratification.md` > Business Core > DBML > FE Core > BE Core > tool/prompt contracts > this plan. `report/` is not an authority and never a sprint prerequisite. Start with `manifest.json` for scheduling, `coverage-matrix.md` for requirements, `dependency-graph.md` for edges/parallelism, `integration-contract.md` for execution protocol; then open a single `phase<i>_sprint<j>_deep<k>.md` sprint file. Sprint IDs are stable; phase index is engineering capability and **not** business Phase 0–6. `deep` is longest in-phase dependency depth, not priority/effort. Cross-phase deps are explicit.

## Dispatch and acceptance
Compute READY from accepted prerequisite sprint gates, source SHA, no conflicting active write sets/locks, frozen shared contracts and isolated test resources. Never infer safety from matching depth or worktrees alone. Sprints are `PLANNED` until a future agent implements, tests, integrates and passes `gate_<id>`; no plan file is a runtime guarantee. Contract-validation sprints may report `CONTRACT_BLOCKED` and block only affected descendants; do not silently modify SSOT to pass.

`manifest.json` includes a `contract_issues` ledger. P1S1 may be accepted as `INVENTORY_ACCEPTED`; that acceptance does not clear any issue. Affected successors carry issue IDs in `contract_blockers` and must stop until the owner decision is recorded. `generated_write_paths` is distinct from hand-authored implementation paths.

Backend implementation sprints never edit `mod.rs`/`lib.rs`; they test owned files through integration-test `#[path]` modules, while P6S2 performs production wiring from accepted artifacts. This deliberately trades early whole-crate linkage for conflict-free file ownership; no sprint may claim full-app compilation before P6S2.

For each sprint: verify its exact baseline and refs; use narrow `sed`/`rg` reads, official compatible CLI, isolated worktree and resource names; implement only allowed paths; run the future-target commands and tests; record actual exit codes; integrate deterministically. On failed merge preserve branch and accepted commits, identify owner, repair/retest, never mark accepted prematurely. After interruption inspect repository and durable DB receipt before retry, never repeat destructive operations blindly.

## Constraints
256K-token maximum for a subagent: do not read complete SSOT by default. TypeSafe/Jev may advise when authorized and available, never override SSOT/tests or transmit private repository data without permission. No sprint authorizes business-rule changes, physical schema changes beyond the schema-owner sprint, plaintext secrets, auto legacy genesis, external-effect replay, production migration or treating mock UI as durable. Known unresolved contract boundaries: migration provenance, artifact consumer projection/budget, ErrorCode/restore/seal failure, TX05 mapping, TX06 ordering, theme enum/default, scene-version restore API, system_language request-vs-settings precedence, and provider-config persistence semantics. See coverage statuses; authority resolution may create a new baseline and require plan revision before dispatch.
'''
(HERE/'README.md').write_text(readme,encoding='utf-8')

integration=f'''# Multi-agent integration contract

Baseline: `{BASE}`. This protocol is for **future execution only**; no worktree, commit, migration or application run occurred during plan authoring.

1. Verify `git rev-parse HEAD`, clean/understood status, SSOT hashes and approved contract evidence; if drift, stop and revise plan/IDs explicitly, not a quiet rebase.
2. READY = every listed predecessor `SPRINT_ACCEPTED` and its gate accepted, no declared contract blocker, current write sets/locks disjoint from active work, generated contracts frozen, platform/toolchain compatible.
3. Build pairwise C1 file, C2 directory, C3 API, C4 schema, C5 build config, C6 fixture, C7 semantics, C8 runtime, C9 generated output, C10 integration-order comparison for any proposed batch across phases. A same-depth tag is not a proof.
4. Create isolated worktree `worktrees/<sprint_id>-<shortsha>` and branch `impl/<sprint_id>/<shortsha>` from the accepted integrated revision. Record source SHA, author, sprint ID and no-secret provenance in commits; never share mutable tree.
5. Dispatch one sprint file plus listed references/accepted handoffs; enforce its exact allowlist. `Cargo.toml`/lockfile/Tauri config belong P2S1, generated DTO P2S5, SQL/version P2S3, `lib.rs`/all module registration/router/i18n composition P6S2 (P2S1 owns initial crate scaffold then hands off). Implementation sprints use `#[path]` test modules and must not edit `mod.rs` before P6S2. No concurrent owner rewrites these shared paths.
6. Run sprint tests with unique `CARGO_TARGET_DIR`, per-sprint `mktemp` root/SQLite DB and dynamically allocated port; no real app-data path or environment-global mutation. Test commands in sprint files are **future targets**, not evidence of a current implementation.
7. Collect diff, hashes, compile/test/visual logs and test DB postconditions. Scope violations, unknown schema/ABI, missing fixture or `CONTRACT_BLOCKED` cause rejection before merge.
8. Integrate accepted branches in deterministic sprint-ID order on an integration worktree; re-run affected tests, typechecks, security checks, PRAGMA FK/integrity, no regression and visual comparison for FE. Record merged SHA. Independent pass does not imply merged pass.
9. Gate state machine: `PLANNED → SPRINT_IMPLEMENTED → SPRINT_TESTED → SPRINT_INTEGRATED → SPRINT_ACCEPTED`; contract-only P1S1 may also record `INVENTORY_ACCEPTED` without clearing any issue. Only recorded accepted gate releases actual descendants. Do not mark a batch accepted if any integrated check fails. Affected descendants remain blocked by `contract_issue_ids` until the owner evidence is ACCEPTED.
10. On failed integration: preserve individual branches/accepted commits; isolate conflicting interface/file; assign repair to its owner or central integrator; rerun both individual and merged tests. No SSOT rewrite by an agent. If owner decision needed, record issue/affected nodes and halt those descendants only.
11. On interrupted agent: inspect worktree/branch status, actual artifacts and durable receipts; continue/reject based on state, not prior narrative. Never auto-rerun migration, TX or external effect. On rejected patch retain evidence and revalidate predecessor SHA before rescheduling.
12. Validate final all-requirement coverage, 40 IPC commands/3 channels, eight TX, 66-table target, SF-06 historical vs current applicability, pending TX08 independence, Phase3 exact consumer input, Writing Mode Save/Lock, full fault/visual tests; only then record release gate SHA. No unit-test-only release claim.

## Contract escalation and explicit blockers
P1S1: unresolved seal_pending_retry/restore/error enum/TX01/TX05/TX06/theme/default/scene restore/inventory/system_language and provider-config persistence precedence; P1S2: 0001/0002 absent and no approved historical migration path; P1S3: no actual Phase3 consumer. Evidence files cannot approve themselves: require authoritative owner/SSOT update outside implementation sprint, then rebaseline/revalidate DAG. Mandatory requirements with `CONTRACT_VALIDATION_REQUIRED` remain assigned but not implementation-ready.
'''
(HERE/'integration-contract.md').write_text(integration,encoding='utf-8')
print(json.dumps({'sprints':len(S),'requirements':len(R),'gates':len(G),'pairs':len(pairs),'critical_path':critical},ensure_ascii=False))
