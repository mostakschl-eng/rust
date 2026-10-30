const fs = require('fs');
const path = require('path');

const rootDir = path.resolve(__dirname, '..');

const steps = [
  {
    id: 'step_1',
    number: 1,
    title: 'Step 1: Core Foundations',
    subtitle: '8 Levels • 80 Lessons • 8 Milestone Projects',
    levels: [
      { id: '01_baby_steps', num: 1, name: 'Level 1: Absolute Baby Steps', range: '01–10' },
      { id: '02_types_and_math', num: 2, name: 'Level 2: Basic Types & Arithmetic', range: '11–20' },
      { id: '03_logic_and_loops', num: 3, name: 'Level 3: Logic, Choices & Loops', range: '21–30' },
      { id: '04_collections', num: 4, name: 'Level 4: Collections & Data', range: '31–40' },
      { id: '05_structs_and_enums', num: 5, name: 'Level 5: Structs, Enums & Patterns', range: '41–50' },
      { id: '06_ownership_and_memory', num: 6, name: 'Level 6: Ownership & Memory', range: '51–60' },
      { id: '07_errors_and_io', num: 7, name: 'Level 7: Errors & System I/O', range: '61–70' },
      { id: '08_advanced_and_concurrency', num: 8, name: 'Level 8: Advanced & Concurrency', range: '71–80' }
    ]
  },
  {
    id: 'step_2',
    number: 2,
    title: 'Step 2: Universal Production Crates',
    subtitle: 'serde, tokio, anyhow',
    guideFile: 'lessons/step_2_universal_crates/README.md',
    taskReadme: 'tasks/step_2_universal_crates/README.md'
  },
  {
    id: 'step_3',
    number: 3,
    title: 'Step 3: Specialization Career Tracks',
    subtitle: 'Web APIs, CLI Tools, Systems',
    guideFile: 'lessons/step_3_specialization_tracks/README.md',
    taskReadme: 'tasks/step_3_specialization_tracks/README.md'
  },
  {
    id: 'step_4',
    number: 4,
    title: 'Step 4: Portfolio Projects & CI/CD',
    subtitle: '3 Full Apps & Production Standards',
    guideFile: 'lessons/step_4_production_and_projects/README.md',
    taskReadme: 'tasks/step_4_production_and_projects/README.md'
  }
];

const allLessons = [];

// Process Step 1 Lessons, Tasks, and Milestone Projects
const step1 = steps[0];
for (const lvl of step1.levels) {
  const lessonDir = path.join(rootDir, 'lessons', 'step_1_core_foundations', lvl.id);
  const taskDir = path.join(rootDir, 'tasks', 'step_1_core_foundations', lvl.id);

  if (!fs.existsSync(lessonDir)) continue;

  const files = fs.readdirSync(lessonDir).filter(f => f.endsWith('.md')).sort();
  const taskDirs = fs.existsSync(taskDir) ? fs.readdirSync(taskDir) : [];

  // Add 10 regular lessons
  for (const f of files) {
    const fullPath = path.join(lessonDir, f);
    const content = fs.readFileSync(fullPath, 'utf8');
    const firstLine = content.split(/\r?\n/)[0] || '';
    const titleMatch = firstLine.replace(/^#\s*(Lesson\s*\d+:\s*)?/, '').trim();
    const title = titleMatch || f.replace(/\.md$/, '');

    const numMatch = f.match(/^(\d+)/);
    const num = numMatch ? parseInt(numMatch[1], 10) : 0;
    const padNum = String(num).padStart(2, '0');

    const matchingTask = taskDirs.find(t => t.startsWith(`task_${padNum}`));
    let taskFolderRel = '';
    let taskBrief = '';
    let starterCode = '';
    let solutionCode = '';
    let solutionExplain = '';

    if (matchingTask) {
      const fullTaskPath = path.join(taskDir, matchingTask);
      taskFolderRel = `tasks/step_1_core_foundations/${lvl.id}/${matchingTask}`;

      const taskMd = path.join(fullTaskPath, 'TASK.md');
      if (fs.existsSync(taskMd)) taskBrief = fs.readFileSync(taskMd, 'utf8');

      const mainRs = path.join(fullTaskPath, 'src', 'main.rs');
      if (fs.existsSync(mainRs)) starterCode = fs.readFileSync(mainRs, 'utf8');

      const solRs = path.join(fullTaskPath, 'solution', 'solution.rs');
      if (fs.existsSync(solRs)) solutionCode = fs.readFileSync(solRs, 'utf8');

      const solMd = path.join(fullTaskPath, 'solution', 'explain.md');
      if (fs.existsSync(solMd)) solutionExplain = fs.readFileSync(solMd, 'utf8');
    }

    allLessons.push({
      id: `lesson_${padNum}`,
      isProject: false,
      num,
      padNum,
      title,
      stepId: 'step_1',
      stepName: step1.title,
      levelId: lvl.id,
      levelNum: lvl.num,
      levelName: lvl.name,
      filename: f,
      content,
      taskFolder: taskFolderRel,
      taskBrief,
      starterCode,
      solutionCode,
      solutionExplain
    });
  }

  // Check for Milestone Project in this level
  const projectDirName = taskDirs.find(t => t.startsWith('project_'));
  if (projectDirName) {
    const fullProjPath = path.join(taskDir, projectDirName);
    const taskFolderRel = `tasks/step_1_core_foundations/${lvl.id}/${projectDirName}`;

    let taskBrief = '';
    const taskMd = path.join(fullProjPath, 'TASK.md');
    if (fs.existsSync(taskMd)) taskBrief = fs.readFileSync(taskMd, 'utf8');

    let starterCode = '';
    const mainRs = path.join(fullProjPath, 'src', 'main.rs');
    if (fs.existsSync(mainRs)) starterCode = fs.readFileSync(mainRs, 'utf8');

    let solutionCode = '';
    const solRs = path.join(fullProjPath, 'solution', 'solution.rs');
    if (fs.existsSync(solRs)) solutionCode = fs.readFileSync(solRs, 'utf8');

    let solutionExplain = '';
    const solMd = path.join(fullProjPath, 'solution', 'explain.md');
    if (fs.existsSync(solMd)) solutionExplain = fs.readFileSync(solMd, 'utf8');

    const firstLine = taskBrief.split(/\r?\n/)[0] || '';
    const projTitle = firstLine.replace(/^#\s*/, '').trim() || `Milestone Project ${lvl.num}`;

    allLessons.push({
      id: `project_0${lvl.num}`,
      isProject: true,
      num: 900 + lvl.num,
      padNum: `P${lvl.num}`,
      title: `⭐ ${projTitle}`,
      stepId: 'step_1',
      stepName: step1.title,
      levelId: lvl.id,
      levelNum: lvl.num,
      levelName: lvl.name,
      filename: 'TASK.md',
      content: taskBrief,
      taskFolder: taskFolderRel,
      taskBrief,
      starterCode,
      solutionCode,
      solutionExplain
    });
  }
}

// Process Steps 2, 3, 4
const extraStepsData = [];
for (let i = 1; i < steps.length; i++) {
  const st = steps[i];
  const gPath = path.join(rootDir, st.guideFile);
  const tPath = path.join(rootDir, st.taskReadme);

  const guideContent = fs.existsSync(gPath) ? fs.readFileSync(gPath, 'utf8') : '';
  const taskContent = fs.existsSync(tPath) ? fs.readFileSync(tPath, 'utf8') : '';

  extraStepsData.push({
    id: st.id,
    number: st.number,
    title: st.title,
    subtitle: st.subtitle,
    guideContent,
    taskContent
  });
}

const data = {
  steps,
  lessons: allLessons,
  extraSteps: extraStepsData
};

const outputJs = `window.PORTAL_DATA = ${JSON.stringify(data)};\n`;
const outputPath = path.join(rootDir, 'portal_data.js');
fs.writeFileSync(outputPath, outputJs, 'utf8');

console.log(`Generated portal_data.js successfully! Total entries: ${allLessons.length} (80 lessons + 8 milestone projects), data size: ${(outputJs.length / 1024).toFixed(1)} KB`);
