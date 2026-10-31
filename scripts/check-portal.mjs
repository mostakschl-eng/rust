import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import React from 'react';
import { renderToStaticMarkup } from 'react-dom/server';
import App from '../src/App.tsx';
import { clampPosition } from '../src/practice-window.tsx';

const data = JSON.parse(await readFile(new URL('../src/generated/content.json', import.meta.url), 'utf8'));
globalThis.location = { hash: '' };
globalThis.localStorage = { getItem: () => null };
const render = () => renderToStaticMarkup(React.createElement(App));
const home = render();
assert.equal((home.match(/class="step-heading[^\"]*" aria-expanded="false"/g) || []).length, 4);
assert.ok(!home.includes('class="step-content"'), 'Steps must start collapsed');
assert.ok(home.includes('Rust <b>Learn</b>'));
assert.ok(!home.includes('Rust Master'));
assert.ok(home.includes('id="open-practice"'));
assert.ok(!home.includes('id="tab-Practice"'));
assert.ok(!home.includes('role="dialog"'));
assert.deepEqual(clampPosition(-20,900,580,650,1200,800),{x:0,y:150});
assert.deepEqual(clampPosition(800,80,580,650,1200,800),{x:620,y:80});
assert.deepEqual(clampPosition(20,20,580,650,320,480),{x:0,y:0});
const projects = data.entries.filter(entry => entry.isProject);
assert.equal(projects.length, 8);
for (const project of projects) {
  assert.equal(project.taskBrief, await readFile(project.taskSourcePath, 'utf8'));
  assert.equal(project.solutionCode, await readFile(`${project.taskFolder}/solution/solution.rs`, 'utf8'));
  assert.equal(project.solutionExplain, await readFile(project.explainSourcePath, 'utf8'));
  location.hash = `#${project.id}/Task`;
  const brief = render();
  assert.ok(brief.includes('aria-selected="true" aria-controls="lesson-panel" tabindex="0">◇ Project'));
  assert.ok(!brief.includes('Your Rust workspace'));
  assert.ok(!brief.includes('Reveal project solution'));
  assert.ok(!brief.includes('Reveal task solution'));
  assert.ok(!brief.includes('class="project-nav"'));
  assert.ok(brief.includes('Mark project complete'));
  const host=data.entries.filter(entry=>!entry.isProject && entry.levelId===project.levelId).at(-1);
  assert.ok(brief.includes('id="tab-Project"'));
  location.hash=`#${host.id}/Project`;
  assert.equal(render(),brief,'Legacy project links must open the embedded project tab');
  assert.ok(brief.includes('class="markdown"'), 'Project brief must render');
  location.hash = `#${project.id}/Solution`;
  const solutions=render();
  assert.ok(solutions.includes('Reveal project solution'), 'Project solutions must stay spoiler protected');
  assert.ok(solutions.includes('Reveal task solution'));
  assert.ok(!solutions.includes('Your Rust workspace'));
  location.hash=`#${project.id}/Practice`;
  const practice=render();
  assert.ok(practice.includes('role="dialog"'));
  assert.ok(practice.includes('aria-modal="false"'));
  assert.ok(practice.includes('Maximize practice'));
  assert.ok(practice.includes('Minimize practice'));
  assert.ok(practice.includes('Close practice'));
  assert.ok(!practice.includes('Choose practice task or project'));
  assert.ok(!practice.includes('<select'));
  assert.ok(!practice.includes('// TODO:'));
  assert.ok(practice.includes('rust-code-editor'));
  assert.ok(!practice.includes('workspace-toolbar'));
  assert.ok(!practice.includes('<textarea'));
  assert.ok(practice.includes('Program output'));
  assert.ok(practice.includes('▶ Run'));
  assert.ok(!practice.includes('Download .rs'));
  assert.ok(!practice.includes('Reset starter'));
  assert.ok(!practice.includes('Your Rust workspace'));
  assert.ok(!practice.includes('class="notice"'));
}
for(const lesson of data.entries.filter(entry=>!entry.isProject)) {
  location.hash=`#${lesson.id}/Lesson`;
  const page=render();
  const isCheckpoint=data.entries.filter(entry=>!entry.isProject && entry.levelId===lesson.levelId).at(-1).id===lesson.id;
  assert.equal(page.includes('id="tab-Project"'),isCheckpoint);
  location.hash=`#${lesson.id}/Solution`;
  const solution=render();
  assert.ok(solution.includes('Reveal task solution'));
  assert.equal(solution.includes('Reveal project solution'),isCheckpoint);
  assert.ok(!solution.includes('id="tab-Practice"'));
  assert.ok(!solution.includes('role="dialog"'));
}
console.log('Portal checks passed: 80 lesson pages, eight project briefs, combined solution sections, global non-modal practice window, legacy practice links and viewport movement bounds.');

globalThis.localStorage = {getItem:()=>JSON.stringify({version:1,drafts:{scratchpad:'fn main() { println!("My scratchpad"); }',lesson_10:'student task attempt'},completed:[],theme:'dark'})};
for(const hash of ['#lesson_01/Practice','#lesson_10/Practice','#project_01/Practice']) {
  location.hash=hash;
  const page=render();
  assert.ok(page.includes('rust-code-editor'),'Scratchpad editor must stay available across task/project routes');
  assert.ok(!page.includes('student task attempt'),'Old task drafts must not replace the scratchpad');
}
console.log('Scratchpad checks passed: no selector, independent saved code across task/project routes.');
