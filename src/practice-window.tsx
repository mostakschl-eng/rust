import { useEffect, useRef, useState, type ReactNode } from 'react';

export function clampPosition(x:number,y:number,width:number,height:number,viewportWidth:number,viewportHeight:number) {
  return {x:Math.max(0,Math.min(x,viewportWidth-width)),y:Math.max(0,Math.min(y,viewportHeight-height))};
}

export default function PracticeWindow({children,onClose,onRun,busy}:{children:ReactNode;onClose:()=>void;onRun:()=>void;busy:boolean}) {
  const panel=useRef<HTMLElement>(null);
  const drag=useRef<{x:number;y:number;left:number;top:number} | null>(null);
  const [position,setPosition]=useState(()=>({x:typeof window==='undefined'?24:Math.max(0,window.innerWidth-1032),y:80}));
  const [minimized,setMinimized]=useState(false);
  const [maximized,setMaximized]=useState(false);
  function move(x:number,y:number) {
    const rect=panel.current?.getBoundingClientRect();
    if(rect)setPosition(clampPosition(x,y,rect.width,rect.height,window.innerWidth,window.innerHeight));
  }
  useEffect(()=>{
    const fit=()=>{if(maximized)return;const rect=panel.current?.getBoundingClientRect();if(rect)setPosition(previous=>clampPosition(previous.x,previous.y,rect.width,rect.height,window.innerWidth,window.innerHeight));};
    window.addEventListener('resize',fit);
    fit();
    return ()=>window.removeEventListener('resize',fit);
  },[minimized,maximized]);
  return <section ref={panel} className={`practice-window ${minimized?'minimized':''} ${maximized?'maximized':''}`} style={maximized?undefined:{left:position.x,top:position.y}} role="dialog" aria-modal="false" aria-labelledby="practice-window-title" onKeyDown={event=>{if(event.key==='Escape' && event.target instanceof HTMLElement && event.target.tagName!=='TEXTAREA'){event.stopPropagation();onClose();}}}>
    <header className="practice-window-bar">
      <button className="practice-drag" id="practice-window-title" title="Drag to move. Use arrow keys when focused." aria-label="Practice workspace — drag or use arrow keys to move" onPointerDown={event=>{
        if(maximized || event.button!==0)return;
        drag.current={x:event.clientX,y:event.clientY,left:position.x,top:position.y};
        event.currentTarget.setPointerCapture(event.pointerId);
      }} onPointerMove={event=>{if(drag.current)move(drag.current.left+event.clientX-drag.current.x,drag.current.top+event.clientY-drag.current.y);}} onPointerUp={event=>{drag.current=null;if(event.currentTarget.hasPointerCapture(event.pointerId))event.currentTarget.releasePointerCapture(event.pointerId);}} onPointerCancel={()=>{drag.current=null;}} onKeyDown={event=>{
        if(maximized || !['ArrowLeft','ArrowRight','ArrowUp','ArrowDown'].includes(event.key))return;
        event.preventDefault();
        move(position.x+(event.key==='ArrowRight'?20:event.key==='ArrowLeft'?-20:0),position.y+(event.key==='ArrowDown'?20:event.key==='ArrowUp'?-20:0));
      }}>⌘ Practice <span>Drag to move</span></button>
      <button className="workspace-run" disabled={busy} onClick={onRun} title="Run with Rust Playground (sends your code). Ctrl+Enter" aria-keyshortcuts="Control+Enter Meta+Enter">{busy?'Running…':'▶ Run'}</button>
      <button aria-label={minimized?'Restore practice':'Minimize practice'} onClick={()=>{setMinimized(!minimized);setMaximized(false);}}>{minimized?'▢':'−'}</button>
      <button aria-label={maximized?'Restore practice size':'Maximize practice'} onClick={()=>{setMaximized(!maximized);setMinimized(false);}}>{maximized?'❐':'□'}</button>
      <button aria-label="Close practice" onClick={onClose}>×</button>
    </header>
    <div className="practice-window-body" hidden={minimized}>{children}</div>
  </section>;
}
