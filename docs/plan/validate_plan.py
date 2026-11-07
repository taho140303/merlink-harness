"""Read-only structural/coverage/concurrency validator for docs/plan (stdlib only)."""
from __future__ import annotations
import json,re,runpy,subprocess,collections
from pathlib import Path
P=Path(__file__).resolve().parent; ROOT=P.parent.parent
m=runpy.run_path(str(P/'plan_model.py'))
manifest=json.loads((P/'manifest.json').read_text(encoding='utf-8'))
sprints=manifest['sprints']; gates=manifest['integration_gates']; reqs=manifest['requirements']
ids=[s['sprint_id'] for s in sprints]; gid=[g['id'] for g in gates]
assert manifest['schema_version']==1 and manifest['project']=='merlink-harness'
assert manifest['baseline_commit']==m['BASE']==subprocess.check_output(['git','rev-parse','HEAD'],cwd=ROOT,text=True).strip()
assert len(ids)==len(set(ids))==len(m['S'])
assert len(gid)==len(set(gid))==len(ids)
assert len(reqs)==len({r['requirement_id'] for r in reqs})==len(m['R'])
assert sorted(ids)==sorted(x['id'] for x in m['S'])
assert sorted((x['sprint_id'],x['depth']) for x in sprints)==sorted((x['id'],x['deep']) for x in m['S'])
assert sorted(p.name for p in P.glob('phase*_sprint*_deep*.md'))==sorted(x['filename'] for x in sprints)
byid={s['sprint_id']:s for s in sprints}; gg={g['id']:g for g in gates}
sections=[f'## {n}. '+x for n,x in enumerate(['Sprint identity','Objective','Scope boundaries','Authoritative contract snapshot','Prerequisites','Inputs','File ownership','Public interfaces','Implementation requirements','Domain invariants','Failure matrix','Acceptance tests','Database and migration safety','Concurrency contract','Deliverables','Completion criteria','Handoff contract','Resume and failure behavior'],1)]
for s in sprints:
    sid=s['sprint_id']; p=P/s['filename']; text=p.read_text(encoding='utf-8')
    assert re.fullmatch(r'phase[1-9]\d*_sprint[1-9]\d*_deep\d+\.md',p.name)
    assert all(h in text for h in sections),(sid,'missing heading')
    assert f'baseline_commit: {m["BASE"]}' in text and 'status: PLANNED' in text
    assert not any(x in text for x in ['TBD','TODO','Implement as needed','Follow previous discussion','Use the usual approach'])
    assert all(d['sprint_id'] in byid and d['sprint_id']!=sid for d in s['depends_on'])
    assert len(s['depends_on'])==len({d['sprint_id'] for d in s['depends_on']})
    assert all(x in gg for x in s['required_integration_gates']+[s['acceptance_gate']])
    assert s['required_integration_gates']==['gate_'+d['sprint_id'] for d in s['depends_on']]
    for d in s['depends_on']:
        if byid[d['sprint_id']]['phase']==s['phase']:assert byid[d['sprint_id']]['depth']<s['depth']
    expected=0 if not any(byid[d['sprint_id']]['phase']==s['phase'] for d in s['depends_on']) else 1+max(byid[d['sprint_id']]['depth'] for d in s['depends_on'] if byid[d['sprint_id']]['phase']==s['phase'])
    assert s['depth']==expected,(sid,'depth',expected)
    assert 'CONTRACT_BLOCKED' in text and 'No `report/` input' in text
    assert all(rid in text for rid in s['requirement_ids'])
    assert all(x in text for x in ['allowed_write_paths:','forbidden_write_paths:','shared_read_only_paths:','generated_paths:'])
    assert not any(path.startswith('report/') for path in s['allowed_write_paths'])
    for rid in s['requirement_ids']:
        if rid.startswith('REQ-CMD-'):assert 'IPC-CMD-'+rid[-3:] in text
        if rid.startswith('REQ-TX-'):assert 'TX-'+rid[-3:] in text
    assert f'{sid}-F0' in text
# DAG acyclic, no source/sprint requirement owner gaps.
def visit(sid,stack=()):
    assert sid not in stack,(sid,stack)
    for d in byid[sid]['depends_on']:visit(d['sprint_id'],stack+(sid,))
for sid in ids:visit(sid)
for r in reqs:
    owner=r['implementation_owner_sprint']; assert owner in byid
    assert r['verification_owner']==byid[owner]['acceptance_gate']
    assert r['source_anchor'].startswith(r['source_file']+'#L')
    assert r['status'] in ('ASSIGNED','CONTRACT_VALIDATION_REQUIRED','UNRESOLVED')
    assert r['requirement_id'] in byid[owner]['requirement_ids']
    path=ROOT/r['source_file']; assert path.is_file()
    a,b=map(int,re.findall(r'\d+',r['source_anchor'].split('#L',1)[1]));assert 1<=a<=b<=len(path.read_text(encoding='utf-8').splitlines())
for g in gates:
    assert g['sprint_id'] in byid and g['id']==byid[g['sprint_id']]['acceptance_gate']
    assert g['requires']==byid[g['sprint_id']]['required_integration_gates']
    assert g['status']=='PLANNED' and len(g['checks'])>=7
# Explicit pairwise conflict check for every same-phase/depth pair.
pairs=0; unsafe=[]
for i,a in enumerate(sprints):
    for b in sprints[i+1:]:
        if (a['phase'],a['depth'])!=(b['phase'],b['depth']):continue
        pairs+=1
        collide=set(a['allowed_write_paths'])&set(b['allowed_write_paths'])
        locks=set(a['shared_resource_locks'])&set(b['shared_resource_locks'])
        safe=not collide and not locks
        assert (b['sprint_id'] in a['parallel_eligible_with'])==safe
        assert (a['sprint_id'] in b['parallel_eligible_with'])==safe
        if not safe:unsafe.append((a['sprint_id'],b['sprint_id'],collide,locks))
# Shared path across DIFFERENT phases/depths must have a dependency path (bootstrap→integrator).
def ancestors(sid):
    out=set()
    for d in byid[sid]['depends_on']:
        out.add(d['sprint_id']);out.update(ancestors(d['sprint_id']))
    return out
for i,a in enumerate(sprints):
    for b in sprints[i+1:]:
        overlap=set(a['allowed_write_paths'])&set(b['allowed_write_paths'])
        assert not overlap or a['sprint_id'] in ancestors(b['sprint_id']) or b['sprint_id'] in ancestors(a['sprint_id']),(a['sprint_id'],b['sprint_id'],overlap)
# Untracked files are NOT covered by git diff --check: check every planning file directly.
for p in P.iterdir():
    if p.is_file() and not p.name.startswith('.dsh-ops-update-'):
        txt=p.read_text(encoding='utf-8')
        assert txt.endswith('\n'),(p,'missing final newline')
        assert all(line==line.rstrip(' \t') for line in txt.splitlines()),(p,'trailing whitespace')
for p in P.glob('*.py'):
    import ast
    ast.parse(p.read_text(encoding='utf-8'))
# Machine/graph and coverage synchronized.
graph=(P/'dependency-graph.md').read_text(encoding='utf-8');coverage=(P/'coverage-matrix.md').read_text(encoding='utf-8')
for s in sprints:
    assert s['sprint_id'] in graph and s['sprint_id'] in coverage
    for d in s['depends_on']:assert f'{d["sprint_id"]} --> {s["sprint_id"]}' in graph
for r in reqs:assert r['requirement_id'] in coverage
status=collections.Counter(r['status'] for r in reqs)
if __name__=='__main__':
    import sys
    if len(sys.argv)>1:
        assert len(sys.argv)==3 and sys.argv[1]=='--contract-evidence','usage: --contract-evidence docs/plan/evidence/<file>.md'
        e=(ROOT/sys.argv[2]).resolve();assert e.is_file() and e.is_relative_to(P/'evidence')
        body=e.read_text(encoding='utf-8')
        assert re.search(r'^CONTRACT_STATUS: (ACCEPTED|INVENTORY_ACCEPTED)$',body,re.M)
        assert re.search(r'^AUTHORITY_SHA: [0-9a-f]{40}$',body,re.M)
        assert 'ISSUE_ID:' in body and 'SOURCE_ANCHOR:' in body and 'DISPOSITION:' in body
        if 'CONTRACT_STATUS: ACCEPTED' in body:assert 'DISPOSITION: CONTRACT_BLOCKED' not in body
print(json.dumps({'result':'PLAN_STRUCTURE_VALIDATED','baseline':m['BASE'],'phases':len(manifest['phases']),'sprints':len(sprints),'gates':len(gates),'requirements':len(reqs),'status':status,'unowned':0,'missing_acceptance':0,'same_depth_pairs_reviewed':pairs,'unsafe_pairs':unsafe},default=list,ensure_ascii=False))
