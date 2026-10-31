import { lazy, Suspense, useEffect, useRef, useState } from 'react';
import ReactMarkdown from 'react-markdown';
import remarkGfm from 'remark-gfm';
import rehypeRaw from 'rehype-raw';
import rehypeSanitize from 'rehype-sanitize';
import PracticeWindow from './practice-window';
import rawData from './generated/content.json';

type Entry = {id:string;title:string;isProject:boolean;levelId:string;levelName:string;sourcePath:string;taskSourcePath:string;explainSourcePath:string;content:string;taskBrief:string;starterCode:string;solutionCode:string;solutionExplain:string;taskFolder:string};
type Data = {steps:{id:string;title:string;subtitle:string;guideContent?:string;sourcePath?:string;levels?:{id:string;name:string;lessons:string[]}[]}[];entries:Entry[];documents:{id:string;title:string;sourcePath:string;content:string}[]};
const data = rawData as Data;
const RustEditor = lazy(()=>import('./rust-editor'));
const tabs = ['Lesson', 'Task', 'Practice', 'Project', 'Solution'] as const;
type Tab = typeof tabs[number];
const storageKey = 'rust-academy-v1';
type Saved = {version:1;drafts:Record<string,string>;completed:string[];theme:'dark'|'light'};
function load(): Saved {
  const fallback:Saved = {version:1,drafts:{},completed:[],theme:'dark'};
  try {
    const parsed = JSON.parse(localStorage.getItem(storageKey) || 'null');
    if (!parsed || parsed.version !== 1) return fallback;
    const drafts = Object.fromEntries(Object.entries(parsed.drafts && typeof parsed.drafts === 'object' && !Array.isArray(parsed.drafts) ? parsed.drafts : {}).filter(([,value]) => typeof value === 'string')) as Record<string,string>;
    return {version:1,drafts,completed:Array.isArray(parsed.completed)?parsed.completed.filter((id:unknown) => typeof id === 'string' && data.entries.some(entry => entry.id === id)):[],theme:parsed.theme === 'light'?'light':'dark'};
  } catch { return fallback; }
}
function route() {
  try { const [encodedId, tab] = location.hash.slice(1).split('/'); const id = decodeURIComponent(encodedId || ''); const project=data.entries.find(item=>item.id===id && item.isProject); if(project){const host=data.entries.filter(item=>!item.isProject && item.levelId===project.levelId).at(-1);if(host)return {id:host.id,tab:(tab==='Solution'?'Solution':'Project') as Tab};} return {id:id || data.entries[0]?.id || data.steps[0].id,tab:tabs.includes(tab as Tab)?tab as Tab:'Lesson' as Tab}; }
  catch { return {id:data.entries[0]?.id || data.steps[0].id,tab:'Lesson' as Tab}; }
}
function resolvePath(source:string, href:string) {
  const url = new URL(href, `https://course.local/${source}`);
  return {path:decodeURIComponent(url.pathname.slice(1)),fragment:url.hash};
}

export default function App() {
  const [saved,setSaved] = useState(load);
  const [current,setCurrent] = useState(route);
  const [query,setQuery] = useState('');
  const [menu,setMenu] = useState(false);
  const [expandedSteps,setExpandedSteps] = useState<string[]>([]);
  const [unlocked,setUnlocked] = useState<string[]>([]);
  const [saveError,setSaveError] = useState('');
  const [run,setRun] = useState<{busy:boolean;stdout:string;stderr:string;error:string}>({busy:false,stdout:'',stderr:'',error:''});
  const request = useRef<AbortController | null>(null);
  const runSequence = useRef(0);
  const entry = data.entries.find(item => item.id === current.id);
  const step = data.steps.find(item => item.id === current.id);
  const document = data.documents.find(item => item.id === current.id);
  const levelLessons = data.entries.filter(item => !item.isProject && item.levelId===entry?.levelId);
  const project = entry && !entry.isProject && levelLessons.at(-1)?.id===entry.id ? data.entries.find(item=>item.isProject && item.levelId===entry.levelId) : undefined;
  const entryTabs = tabs.filter(tab=>tab!=='Practice' && (tab!=='Project' || project));
  const activeTab = current.tab==='Practice' || (current.tab==='Project' && !project) ? 'Lesson' : current.tab;
  const [practiceOpen,setPracticeOpen] = useState(()=>location.hash.endsWith('/Practice'));
  const code = saved.drafts.scratchpad ?? 'fn main() {\n    \n}\n';
  const lessonEntries = data.entries.filter(item=>!item.isProject);
  const lessonIndex = lessonEntries.findIndex(item=>item.id===(entry?.isProject?levelLessons.at(-1)?.id:entry?.id));
  const complete = entry ? saved.completed.includes(entry.id) : false;
  const lessonDone = data.entries.filter(item => !item.isProject && saved.completed.includes(item.id)).length;
  const projectDone = data.entries.filter(item => item.isProject && saved.completed.includes(item.id)).length;
  useEffect(() => { const listener=()=>setCurrent(route()); window.addEventListener('hashchange',listener);return()=>window.removeEventListener('hashchange',listener); },[]);
  useEffect(() => { window.document.documentElement.dataset.theme=saved.theme;try {localStorage.setItem(storageKey,JSON.stringify(saved));setSaveError('');}catch {setSaveError('Browser storage is unavailable or full. Copy your code somewhere safe.');} },[saved]);
  useEffect(() => { runSequence.current++;request.current?.abort();setRun({busy:false,stdout:'',stderr:'',error:''}); },[practiceOpen]);
  useEffect(() => () => request.current?.abort(),[]);
  function openPractice() {setPracticeOpen(true);}
  function closePractice() {setPracticeOpen(false);window.document.getElementById('open-practice')?.focus();}
  useEffect(()=>{if(location.hash.endsWith('/Practice'))openPractice();},[current.id,current.tab]);
  function navigate(id:string, tab:Tab='Lesson') {if(tab==='Practice')openPractice();location.hash=`${encodeURIComponent(id)}/${tab}`;setMenu(false);window.scrollTo(0,0);}
  function updateCode(value:string) {runSequence.current++;request.current?.abort();setRun({busy:false,stdout:'',stderr:'',error:''});setSaved(previous=>({...previous,drafts:{...previous.drafts,scratchpad:value}}));}
  async function execute() {
    if(run.busy)return;
    const sequence=++runSequence.current;
    const controller=new AbortController();request.current=controller;
    const timeout=window.setTimeout(()=>controller.abort(),30000);
    setRun({busy:true,stdout:'',stderr:'',error:''});
    try {
      const response=await fetch('https://play.rust-lang.org/execute',{method:'POST',headers:{'Content-Type':'application/json'},body:JSON.stringify({channel:'stable',mode:'debug',edition:'2021',crateType:'bin',tests:false,code,backtrace:false}),signal:controller.signal});
      if(!response.ok)throw new Error(`Playground returned HTTP ${response.status}. Try again or use local Cargo.`);
      const result:unknown=await response.json();
      if(!result || typeof result!=='object' || !('stdout' in result) || !('stderr' in result) || typeof result.stdout!=='string' || typeof result.stderr!=='string')throw new Error('Playground returned an unexpected response.');
      if(sequence===runSequence.current)setRun({busy:false,stdout:result.stdout,stderr:result.stderr,error:'success' in result && result.success === false?'The program did not finish successfully. Read stderr below.':''});
    } catch(error) {if(sequence===runSequence.current)setRun({busy:false,stdout:'',stderr:'',error:controller.signal.aborted?'Run timed out after 30 seconds. Try local Cargo.':error instanceof Error?error.message:'Could not reach Playground. Check your connection or use local Cargo.'});}
    finally {window.clearTimeout(timeout);}
  }
  function markdown(content:string,source:string) {
    return <ReactMarkdown remarkPlugins={[remarkGfm]} rehypePlugins={[rehypeRaw,rehypeSanitize]} components={{a:({href,children,...props})=>{
      if(!href)return <span>{children}</span>;
      if(/^(https?:|mailto:)/i.test(href))return <a {...props} href={href} target="_blank" rel="noreferrer">{children}</a>;
      if(href.startsWith('#'))return <a href={href} onClick={event=>{event.preventDefault();window.document.getElementById(`user-content-${href.slice(1)}`)?.scrollIntoView();}}>{children}</a>;
      try {
        let {path,fragment}=resolvePath(source,href); if(path.endsWith('/')){ const first=data.entries.find(item=>item.sourcePath.startsWith(path) || item.taskSourcePath.startsWith(path)); if(first)return <a href={`#${encodeURIComponent(first.id)}/${path.startsWith('tasks/')?'Task':'Lesson'}`} onClick={event=>{event.preventDefault();navigate(first.id,path.startsWith('tasks/')?'Task':'Lesson');}}>{children}</a>; path+='README.md'; }
        const target=data.entries.find(item=>[item.sourcePath,item.taskSourcePath,item.explainSourcePath].includes(path) || `${item.taskFolder}/solution/solution.rs`===path || `${item.taskFolder}/src/main.rs`===path);
        const doc=data.documents.find(item=>item.sourcePath===path);
        const targetStep=data.steps.find(item=>item.sourcePath===path);
        const tab:Tab=target?.taskSourcePath===path?'Task':target?.explainSourcePath===path || path.endsWith('/solution/solution.rs')?'Solution':path.endsWith('/src/main.rs')?'Practice':'Lesson';
        if(target || doc || targetStep)return <a href={`#${encodeURIComponent(target?.id || doc?.id || targetStep?.id || "")}/${tab}`} onClick={event=>{event.preventDefault();navigate((target || doc || targetStep)!.id,tab);if(fragment)setTimeout(()=>window.document.getElementById(`user-content-${fragment.slice(1)}`)?.scrollIntoView(),100);}}>{children}</a>;
        return <a href={`${import.meta.env.BASE_URL}course/${path}`} target="_blank" rel="noreferrer">{children} <small>(repository file)</small></a>;
      }catch{return <span>{children}</span>;}
    },h1:({children})=><h1 id={`user-content-${slug(children)}`}>{children}</h1>,h2:({children})=><h2 id={`user-content-${slug(children)}`}>{children}</h2>,h3:({children})=><h3 id={`user-content-${slug(children)}`}>{children}</h3>}}>{content}</ReactMarkdown>;
  }
  const visible=(item:Entry)=>{
    const checkpoint=data.entries.filter(e=>!e.isProject && e.levelId===item.levelId).at(-1)?.id===item.id;
    const related=checkpoint?data.entries.find(e=>e.isProject && e.levelId===item.levelId):undefined;
    return !query || `${item.title} ${item.levelName} ${item.id} ${related?.title || ''}`.toLowerCase().includes(query.toLowerCase());
  };
  function reference(target:Entry) {
    return unlocked.includes(target.id)?<article className="markdown"><h2>{target.isProject?'Project solution':'Task solution'}</h2><button onClick={()=>setUnlocked(previous=>previous.filter(id=>id!==target.id))}>Hide solution</button><pre><code>{target.solutionCode}</code></pre>{markdown(target.solutionExplain,target.explainSourcePath)}</article>:<div className="solution-shield"><h2>{target.isProject?'Project solution':'Task solution'}</h2><p>Try the instructions and hints first. Reveal the answer when you are ready to compare.</p><button onClick={()=>setUnlocked(previous=>[...previous,target.id])}>Reveal {target.isProject?'project':'task'} solution</button></div>;
  }
  return <><header className="topbar"><button className="menu-button" aria-label="Toggle curriculum" aria-expanded={menu} onClick={()=>setMenu(!menu)}>☰</button><a className="brand" href={`#${data.entries[0]?.id}/Lesson`}><span className="brand-mark" aria-hidden="true">🦀</span>Rust <b>Learn</b></a><span className="header-caption">FROM ZERO TO HERO</span><button id="open-practice" className="practice-launch" aria-haspopup="dialog" aria-expanded={practiceOpen} onClick={()=>openPractice()}>⌘ Practice</button><button className="theme-button" onClick={()=>setSaved({...saved,theme:saved.theme==='dark'?'light':'dark'})} aria-label={`Use ${saved.theme==='dark'?'light':'dark'} theme`}>{saved.theme==='dark'?'☀':'☾'}</button></header>
    <div className="layout"><aside className={menu?'sidebar open':'sidebar'}><div className="sidebar-intro"><span className="eyebrow">YOUR LEARNING PATH</span><h2>One step at a time.</h2><div className="progress-track"><div style={{width:`${(lessonDone+projectDone)/Math.max(data.entries.length,1)*100}%`}}/></div><p>{lessonDone} / 80 lessons · {projectDone} / 8 projects</p><small>Personal checklist · saved in this browser</small></div><label className="search-label"><span className="sr-only">Search curriculum</span><input value={query} onChange={event=>setQuery(event.target.value)} placeholder="Search lessons and projects…" type="search"/></label><nav aria-label="Curriculum">{data.steps.map((item,index)=>{
      const expanded=expandedSteps.includes(item.id) || Boolean(query && item.levels);
      return <section className="step" key={item.id}>
        <button className={`step-heading ${current.id===item.id?'active':''}`} aria-expanded={expanded} aria-controls={`nav-${item.id}`} onClick={()=>setExpandedSteps(previous=>previous.includes(item.id)?previous.filter(id=>id!==item.id):[...previous,item.id])}>
          <span className="step-number">0{index+1}</span><span className="step-title">{item.title.replace(/^Step \d+:\s*/,'').replace(/\s*\([^)]*\)\s*$/,'')}</span><span className="step-chevron" aria-hidden="true">{expanded?'⌄':'›'}</span>
        </button>
        {expanded && <div id={`nav-${item.id}`} className="step-content">
          <button className="lesson-link step-overview" onClick={()=>navigate(item.id)}>Overview{!item.levels?' · Planned outline':''}</button>
          {item.levels?.map(level=><div key={level.id}>
            <details key={query?'search':'browse'} open={query?true:undefined} className="level"><summary>{level.name}<span>{data.entries.filter(e=>e.levelId===level.id && !e.isProject && saved.completed.includes(e.id)).length}/10</span></summary>
              {data.entries.filter(e=>e.levelId===level.id && !e.isProject && visible(e)).map(e=><button className={`lesson-link ${e.id===current.id?'active':''}`} key={e.id} onClick={()=>navigate(e.id)} aria-current={e.id===current.id?'page':undefined}><span className="item-dot">{saved.completed.includes(e.id)?'✓':'○'}</span><span>{e.title}{data.entries.filter(item=>!item.isProject && item.levelId===level.id).at(-1)?.id===e.id && <small className="checkpoint-label">Includes level project</small>}</span></button>)}
            </details>
          </div>)}
        </div>}
      </section>;
    })}{query && !data.entries.some(visible) && <p className="empty">No matching lessons or projects.</p>}</nav><footer className="sidebar-footer">Built for steady progress.<br/>Practice. Understand. Repeat.</footer></aside>
    <main id="main-content"><div className="breadcrumb">RUST LEARNING PATH <span>/</span> {entry?.levelName || 'COURSE GUIDE'}</div><div className="page-heading"><div><span className="eyebrow">{entry?.isProject?'MILESTONE PROJECT':entry?'CORE FOUNDATIONS':'LEARNING ROADMAP'}</span><h1>{entry?.title || step?.title || document?.title || 'Page not found'}</h1><p>{entry?'Read the idea. Try it yourself. Build your understanding.':step?.subtitle}</p></div>{entry && <button className={complete?'completion checked':'completion'} onClick={()=>setSaved(previous=>({...previous,completed:complete?previous.completed.filter(id=>id!==entry.id):[...previous.completed,entry.id]}))} aria-pressed={complete}>{complete?'✓ Completed':'○ Mark complete'}</button>}</div>
    {saveError && <p role="alert" className="notice error">{saveError}</p>}
    {entry?<><div className="tabs" role="tablist" aria-label="Lesson sections">{entryTabs.map((tab,index)=><button key={tab} id={`tab-${tab}`} role="tab" aria-selected={activeTab===tab} aria-controls="lesson-panel" tabIndex={activeTab===tab?0:-1} onClick={()=>navigate(entry.id,tab)} onKeyDown={event=>{if(event.key==='ArrowRight' || event.key==='ArrowLeft'){event.preventDefault();const next=entryTabs[(index+(event.key==='ArrowRight'?1:entryTabs.length-1))%entryTabs.length];navigate(entry.id,next);window.document.getElementById(`tab-${next}`)?.focus();}}}>{{Lesson:'▤',Task:'☑',Practice:'⌘',Project:'◇',Solution:'♧'}[tab]} {entry.isProject?({Lesson:'Overview',Task:'Project',Practice:'Practice',Project:'Project',Solution:'Project solution'}[tab]):tab}</button>)}</div><section id="lesson-panel" role="tabpanel" aria-labelledby={`tab-${activeTab}`} className="content-card">
    {activeTab==='Lesson' && <article className="markdown">{markdown(entry.content,entry.sourcePath)}</article>}
    {activeTab==='Task' && <article className="markdown">{markdown(entry.taskBrief,entry.taskSourcePath)}</article>}
    {activeTab==='Project' && project && <><article className="markdown">{markdown(project.taskBrief,project.taskSourcePath)}</article><button className={saved.completed.includes(project.id)?'completion checked':'completion'} aria-pressed={saved.completed.includes(project.id)} onClick={()=>setSaved(previous=>({...previous,completed:previous.completed.includes(project.id)?previous.completed.filter(id=>id!==project.id):[...previous.completed,project.id]}))}>{saved.completed.includes(project.id)?'✓ Project completed':'○ Mark project complete'}</button></>}

    {activeTab==='Solution' && <div className="solution-options">{reference(entry)}{project && reference(project)}</div>}
    </section><div className="lesson-footer"><button disabled={lessonIndex<=0} onClick={()=>navigate(lessonEntries[lessonIndex-1].id)}>← Previous</button><span>Progress is a personal checklist.</span><button disabled={lessonIndex===lessonEntries.length-1} onClick={()=>navigate(lessonEntries[lessonIndex+1].id)}>Next →</button></div></>:<article className="content-card markdown">{markdown(step?.guideContent || document?.content || (step?.levels?'Choose a level in the sidebar to begin the 80 lessons and eight projects.':'This page could not be found. Choose a lesson in the sidebar.'),step?.sourcePath || document?.sourcePath || 'ROADMAP.md')}</article>}
    </main></div>{practiceOpen && <PracticeWindow onClose={closePractice} onRun={execute} busy={run.busy}>
      <Suspense fallback={<div className="rust-code-editor" role="status">Loading editor…</div>}><RustEditor value={code} onChange={updateCode} onRun={execute}/></Suspense>
      <section className="workspace-output" aria-label="Program output" aria-live="polite" aria-busy={run.busy}>
        <div className="workspace-output-title">Output <span>{run.busy?'Running…':'Ctrl+Enter to run'}</span></div>
        {run.error && <pre className="error" role="alert">{run.error}</pre>}
        <pre>{run.stdout || (run.busy?'Running…':'')}</pre>
        {run.stderr && <><span className="workspace-stderr-label">Compiler / stderr</span><pre className="workspace-stderr">{run.stderr}</pre></>}
      </section>
      <div className="workspace-status"><span title="Your code runs on the external Rust Playground service. Interactive input, local files, multi-file projects and dependencies need local Cargo.">Rust Playground · single file</span><span title="Tab indents. Press Escape, then Tab to leave the editor.">UTF-8 · Rust</span></div>
    </PracticeWindow>}</>;
}
function slug(value:React.ReactNode):string {return String(Array.isArray(value)?value.map(item=>typeof item==='string'?item:'').join(''):value).toLowerCase().replace(/[^\p{L}\p{N}\s_-]/gu,'').replace(/\s/g,'-');}

