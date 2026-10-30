import { useEffect, useRef, useState } from 'react';
import ReactMarkdown from 'react-markdown';
import remarkGfm from 'remark-gfm';
import rehypeRaw from 'rehype-raw';
import rehypeSanitize from 'rehype-sanitize';
import rawData from './generated/content.json';

type Entry = {id:string;title:string;isProject:boolean;levelId:string;levelName:string;sourcePath:string;taskSourcePath:string;explainSourcePath:string;content:string;taskBrief:string;starterCode:string;solutionCode:string;solutionExplain:string;taskFolder:string};
type Data = {steps:{id:string;title:string;subtitle:string;guideContent?:string;sourcePath?:string;levels?:{id:string;name:string;lessons:string[]}[]}[];entries:Entry[];documents:{id:string;title:string;sourcePath:string;content:string}[]};
const data = rawData as Data;
const tabs = ['Lesson', 'Task', 'Practice', 'Solution'] as const;
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
  try { const [encodedId, tab] = location.hash.slice(1).split('/'); const id = decodeURIComponent(encodedId || ''); return {id:id || data.entries[0]?.id || data.steps[0].id,tab:tabs.includes(tab as Tab)?tab as Tab:'Lesson' as Tab}; }
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
  const [unlocked,setUnlocked] = useState<string[]>([]);
  const [saveError,setSaveError] = useState('');
  const [run,setRun] = useState<{busy:boolean;stdout:string;stderr:string;error:string}>({busy:false,stdout:'',stderr:'',error:''});
  const request = useRef<AbortController | null>(null);
  const runSequence = useRef(0);
  const editor = useRef<HTMLTextAreaElement>(null);
  const entry = data.entries.find(item => item.id === current.id);
  const step = data.steps.find(item => item.id === current.id);
  const document = data.documents.find(item => item.id === current.id);
  const code = entry ? saved.drafts[entry.id] ?? entry.starterCode : '';
  const complete = entry ? saved.completed.includes(entry.id) : false;
  const lessonDone = data.entries.filter(item => !item.isProject && saved.completed.includes(item.id)).length;
  const projectDone = data.entries.filter(item => item.isProject && saved.completed.includes(item.id)).length;
  useEffect(() => { const listener=()=>setCurrent(route()); window.addEventListener('hashchange',listener);return()=>window.removeEventListener('hashchange',listener); },[]);
  useEffect(() => { window.document.documentElement.dataset.theme=saved.theme;try {localStorage.setItem(storageKey,JSON.stringify(saved));setSaveError('');}catch {setSaveError('Browser storage is unavailable or full. Download your code to keep a copy.');} },[saved]);
  useEffect(() => { runSequence.current++;request.current?.abort();setRun({busy:false,stdout:'',stderr:'',error:''}); },[current.id]);
  useEffect(() => () => request.current?.abort(),[]);
  function navigate(id:string, tab:Tab='Lesson') {location.hash=`${encodeURIComponent(id)}/${tab}`;setMenu(false);window.scrollTo(0,0);}
  function updateCode(value:string) {runSequence.current++;request.current?.abort();setRun({busy:false,stdout:'',stderr:'',error:''});if(entry)setSaved(previous=>({...previous,drafts:{...previous.drafts,[entry.id]:value}}));}
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
  function download() {if(!entry)return;const url=URL.createObjectURL(new Blob([code],{type:'text/plain;charset=utf-8'}));const link=window.document.createElement('a');link.href=url;link.download=`${entry.id}.rs`;link.click();setTimeout(()=>URL.revokeObjectURL(url),1000);}
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
  const visible=(item:Entry)=>!query || `${item.title} ${item.levelName} ${item.id}`.toLowerCase().includes(query.toLowerCase());
  return <><header className="topbar"><button className="menu-button" aria-label="Toggle curriculum" aria-expanded={menu} onClick={()=>setMenu(!menu)}>☰</button><a className="brand" href={`#${data.entries[0]?.id}/Lesson`}><span className="brand-mark">R</span>Rust Master <b>Academy</b></a><span className="header-caption">FROM ZERO TO HERO</span><button className="theme-button" onClick={()=>setSaved({...saved,theme:saved.theme==='dark'?'light':'dark'})} aria-label={`Use ${saved.theme==='dark'?'light':'dark'} theme`}>{saved.theme==='dark'?'☀':'☾'}</button></header>
    <div className="layout"><aside className={menu?'sidebar open':'sidebar'}><div className="sidebar-intro"><span className="eyebrow">YOUR LEARNING PATH</span><h2>One step at a time.</h2><div className="progress-track"><div style={{width:`${(lessonDone+projectDone)/Math.max(data.entries.length,1)*100}%`}}/></div><p>{lessonDone} / 80 lessons · {projectDone} / 8 projects</p><small>Personal checklist · saved in this browser</small></div><label className="search-label"><span className="sr-only">Search curriculum</span><input value={query} onChange={event=>setQuery(event.target.value)} placeholder="Search lessons…" type="search"/></label><nav aria-label="Curriculum">{data.steps.map((item,index)=><section className="step" key={item.id}><button className={`step-heading ${current.id===item.id?'active':''}`} onClick={()=>navigate(item.id)}><span className="step-number">0{index+1}</span><span>{item.title.replace(/^Step \d+:\s*/,'')}<small>{item.subtitle}</small></span>{!item.levels && <span className="outline-tag">Outline</span>}</button>{item.levels?.map(level=><details key={level.id} open={query || entry?.levelId===level.id?true:undefined} className="level"><summary>{level.name}<span>{data.entries.filter(e=>e.levelId===level.id && saved.completed.includes(e.id)).length}/11</span></summary>{data.entries.filter(e=>e.levelId===level.id && visible(e)).map(e=><button className={`lesson-link ${e.id===current.id?'active':''} ${e.isProject?'project':''}`} key={e.id} onClick={()=>navigate(e.id)} aria-current={e.id===current.id?'page':undefined}><span className="item-dot">{saved.completed.includes(e.id)?'✓':e.isProject?'◇':'○'}</span><span>{e.title}</span></button>)}</details>)}</section>)}{query && !data.entries.some(visible) && <p className="empty">No matching lessons.</p>}</nav><footer className="sidebar-footer">Built for steady progress.<br/>Practice. Understand. Repeat.</footer></aside>
    <main id="main-content"><div className="breadcrumb">RUST LEARNING PATH <span>/</span> {entry?.levelName || 'COURSE GUIDE'}</div><div className="page-heading"><div><span className="eyebrow">{entry?.isProject?'MILESTONE PROJECT':entry?'CORE FOUNDATIONS':'LEARNING ROADMAP'}</span><h1>{entry?.title || step?.title || document?.title || 'Page not found'}</h1><p>{entry?'Read the idea. Try it yourself. Build your understanding.':step?.subtitle}</p></div>{entry && <button className={complete?'completion checked':'completion'} onClick={()=>setSaved(previous=>({...previous,completed:complete?previous.completed.filter(id=>id!==entry.id):[...previous.completed,entry.id]}))} aria-pressed={complete}>{complete?'✓ Completed':'○ Mark complete'}</button>}</div>
    {saveError && <p role="alert" className="notice error">{saveError}</p>}
    {entry?<><div className="tabs" role="tablist" aria-label="Lesson sections">{tabs.map((tab,index)=><button key={tab} id={`tab-${tab}`} role="tab" aria-selected={current.tab===tab} aria-controls="lesson-panel" tabIndex={current.tab===tab?0:-1} onClick={()=>navigate(entry.id,tab)} onKeyDown={event=>{if(event.key==='ArrowRight' || event.key==='ArrowLeft'){event.preventDefault();const next=tabs[(index+(event.key==='ArrowRight'?1:3))%4];navigate(entry.id,next);window.document.getElementById(`tab-${next}`)?.focus();}}}>{['▤','☑','⌘','♧'][index]} {tab}{tab==='Solution' && !unlocked.includes(entry.id)?' · Locked':''}</button>)}</div><section id="lesson-panel" role="tabpanel" aria-labelledby={`tab-${current.tab}`} className="content-card">
    {current.tab==='Lesson' && <article className="markdown">{markdown(entry.content,entry.sourcePath)}</article>}
    {current.tab==='Task' && <article className="markdown">{markdown(entry.taskBrief,entry.taskSourcePath)}</article>}
    {current.tab==='Practice' && <div className="practice"><div className="practice-heading"><div><h2>Your Rust workspace</h2><p>Your edits stay in this browser. Your course files stay unchanged.</p></div><span className="language-badge">Rust · main.rs</span></div><div className="notice">Run sends this code to the external Rust Playground service. It supports a single file here. Interactive stdin, local files, multi-file projects and extra dependencies need local Cargo.</div><label htmlFor="rust-editor" className="editor-label">Your code <span>Tab inserts four spaces · Esc then Tab leaves the editor</span></label><textarea ref={editor} id="rust-editor" className="code-editor" spellCheck={false} autoCapitalize="off" autoCorrect="off" value={code} onChange={event=>updateCode(event.target.value)} onKeyDown={event=>{if(event.key==='Tab' && !event.shiftKey && event.currentTarget.dataset.escape!=='true'){event.preventDefault();const start=event.currentTarget.selectionStart,end=event.currentTarget.selectionEnd;updateCode(`${code.slice(0,start)}    ${code.slice(end)}`);requestAnimationFrame(()=>editor.current?.setSelectionRange(start+4,start+4));}event.currentTarget.dataset.escape=event.key==='Escape'?'true':'false';}}/><div className="editor-toolbar"><button className="primary" disabled={run.busy} onClick={execute}>{run.busy?'Running…':'▶ Run code'}</button><button onClick={download}>↓ Download .rs</button><button className="reset" onClick={()=>{if(window.confirm('Replace your saved code for this lesson with its starter? Download a copy first if you want to keep it.'))updateCode(entry.starterCode);}}>Reset starter</button></div><p className="local-command">For the full task, open <code>{entry.taskFolder}</code> in your terminal and run <code>cargo run</code>.</p><section className="output" aria-live="polite" aria-busy={run.busy}><div className="output-title">OUTPUT <span>{run.busy?'Compiling and running…':'Rust Playground'}</span></div>{run.error && <p className="error" role="alert">{run.error}</p>}<h3>stdout</h3><pre>{run.stdout || (run.busy?'Waiting for result…':'No output yet.')}</pre>{run.stderr && <><h3>stderr / compiler messages</h3><pre>{run.stderr}</pre></>}</section><p className="muted">Running code never marks a lesson complete. Review your result and use the checklist yourself.</p></div>}
    {current.tab==='Solution' && (unlocked.includes(entry.id)?<article className="markdown"><h2>Reference solution</h2><pre><code>{entry.solutionCode}</code></pre>{markdown(entry.solutionExplain,entry.explainSourcePath)}</article>:<div className="solution-shield"><span className="shield-icon">♧</span><span className="eyebrow">GIVE YOURSELF A CHANCE</span><h2>Your best learning happens while trying.</h2><p>Start with the task and its hidden hints. Reveal the reference when you are stuck or ready to compare your work.</p><button onClick={()=>setUnlocked([...unlocked,entry.id])}>Reveal reference solution</button><small>This shield avoids accidental spoilers. It is not access control.</small></div>)}
    </section><div className="lesson-footer"><button disabled={data.entries.indexOf(entry)===0} onClick={()=>navigate(data.entries[data.entries.indexOf(entry)-1].id)}>← Previous</button><span>Progress is a personal checklist.</span><button disabled={data.entries.indexOf(entry)===data.entries.length-1} onClick={()=>navigate(data.entries[data.entries.indexOf(entry)+1].id)}>Next lesson →</button></div></>:<article className="content-card markdown">{markdown(step?.guideContent || document?.content || (step?.levels?'Choose a level in the sidebar to begin the 80 lessons and eight projects.':'This page could not be found. Choose a lesson in the sidebar.'),step?.sourcePath || document?.sourcePath || 'ROADMAP.md')}</article>}
    </main></div></>;
}
function slug(value:React.ReactNode):string {return String(Array.isArray(value)?value.map(item=>typeof item==='string'?item:'').join(''):value).toLowerCase().replace(/[^\p{L}\p{N}\s_-]/gu,'').replace(/\s/g,'-');}
