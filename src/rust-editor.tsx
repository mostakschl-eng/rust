import { useEffect, useRef } from 'react';
import { EditorView, basicSetup } from 'codemirror';
import { keymap } from '@codemirror/view';
import { indentWithTab } from '@codemirror/commands';
import { rust } from '@codemirror/lang-rust';
import { oneDark } from '@codemirror/theme-one-dark';

export default function RustEditor({value,onChange,onRun}:{value:string;onChange:(code:string)=>void;onRun:()=>void}) {
  const host=useRef<HTMLDivElement>(null);
  const view=useRef<EditorView | null>(null);
  const callbacks=useRef({onChange,onRun});
  callbacks.current={onChange,onRun};
  const initialValue=useRef(value);
  useEffect(()=>{
    if(!host.current)return;
    const editor=new EditorView({
      parent:host.current,
      doc:initialValue.current,
      extensions:[
        keymap.of([{key:'Mod-Enter',run:()=>{callbacks.current.onRun();return true;}},indentWithTab]),
        basicSetup,rust(),oneDark,
        EditorView.contentAttributes.of({'aria-label':'Rust code'}),
        EditorView.updateListener.of(update=>{if(update.docChanged)callbacks.current.onChange(update.state.doc.toString());}),
      ],
    });
    view.current=editor;
    return ()=>{view.current=null;editor.destroy();};
  },[]);
  useEffect(()=>{
    const editor=view.current;
    if(editor && editor.state.doc.toString()!==value)editor.dispatch({changes:{from:0,to:editor.state.doc.length,insert:value}});
  },[value]);
  return <div ref={host} className="rust-code-editor"/>;
}
