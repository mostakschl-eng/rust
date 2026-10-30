import { promises as fs, watch } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const portal = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const repository = portal;
const excluded = new Set(['.git', 'node_modules', 'target', 'dist', 'learning-portal', 'public', 'legacy']);
const slash = value => value.split(path.sep).join('/');
const heading = (content, fallback) => content.match(/^#\s+(.+)\r?$/m)?.[1].trim() || fallback;

async function filesUnder(root, prefix = '') {
  const files = [];
  for (const item of await fs.readdir(path.join(root, prefix), { withFileTypes: true })) {
    if (excluded.has(item.name) || item.name.startsWith('.')) continue;
    const relative = path.join(prefix, item.name);
    // Do not follow symlinks outside the learning repository.
    if (item.isDirectory()) files.push(...await filesUnder(root, relative));
    else if (item.isFile()) files.push(slash(relative));
  }
  return files.sort();
}

function outputPath(base, relative) {
  const destination = path.resolve(base, relative);
  if (!destination.startsWith(base + path.sep)) throw new Error(`Unsafe output path: ${relative}`);
  return destination;
}

export async function generateContent({ root = repository, output = portal } = {}) {
  root = path.resolve(root);
  output = path.resolve(output);
  const files = await filesUnder(root);
  const available = new Set(files);
  const read = async relative => {
    if (!available.has(relative)) throw new Error(`Missing required course file: ${relative}`);
    return fs.readFile(path.join(root, relative), 'utf8');
  };
  const documents = await Promise.all(files.filter(file => /\.md$/i.test(file)).map(async sourcePath => {
    const content = await read(sourcePath);
    return { id: `doc:${sourcePath}`, title: heading(content, path.basename(sourcePath)), sourcePath, content };
  }));
  const entries = [];
  const ids = new Set();
  const foundation = 'lessons/step_1_core_foundations';
  const levelDirectories = (await fs.readdir(path.join(root, foundation), { withFileTypes: true }))
    .filter(item => item.isDirectory() && /^\d+_/.test(item.name)).map(item => item.name).sort();
  if (levelDirectories.length !== 8) throw new Error(`Expected eight foundation levels, found ${levelDirectories.length}`);
  const levels = [];
  for (const directory of levelDirectories) {
    const levelId = directory.match(/^\d+/)[0];
    const levelName = directory.replace(/^\d+_/, '').replaceAll('_', ' ');
    const lessons = [];
    const taskRoot = `tasks/step_1_core_foundations/${directory}`;
    const taskDirectories = (await fs.readdir(path.join(root, taskRoot), { withFileTypes: true }))
      .filter(item => item.isDirectory()).map(item => item.name).sort();
    const add = async (number, taskDirectory, sourcePath, isProject) => {
      const id = `${isProject ? 'project' : 'lesson'}_${number}`;
      if (ids.has(id)) throw new Error(`Duplicate course entry: ${id}`);
      ids.add(id);
      const taskFolder = `${taskRoot}/${taskDirectory}`;
      const taskSourcePath = `${taskFolder}/TASK.md`;
      const explainSourcePath = `${taskFolder}/solution/explain.md`;
      const content = await read(sourcePath);
      await read(`${taskFolder}/Cargo.toml`);
      entries.push({ id, title: heading(content, id), isProject, levelId, levelName, sourcePath,
        taskSourcePath, explainSourcePath, content, taskBrief: await read(taskSourcePath),
        starterCode: await read(`${taskFolder}/src/main.rs`), solutionCode: await read(`${taskFolder}/solution/solution.rs`),
        solutionExplain: await read(explainSourcePath), taskFolder });
      lessons.push(id);
    };
    for (const sourcePath of files.filter(file => file.startsWith(`${foundation}/${directory}/`) && /^\d+_[^/]+\.md$/.test(file.split('/').at(-1)))) {
      const number = path.basename(sourcePath).match(/^\d+/)[0];
      const matches = taskDirectories.filter(name => new RegExp(`^task_${number}_`).test(name));
      if (matches.length !== 1) throw new Error(`Expected one task for ${sourcePath}, found ${matches.length}`);
      await add(number, matches[0], sourcePath, false);
    }
    for (const directory of taskDirectories.filter(name => /^project_\d+_/.test(name))) {
      await add(directory.match(/^project_(\d+)_/)[1], directory, `${taskRoot}/${directory}/TASK.md`, true);
    }
    levels.push({ id: levelId, name: levelName, lessons });
  }
  const stepDirectories = ['step_1_core_foundations', 'step_2_universal_crates', 'step_3_specialization_tracks', 'step_4_production_and_projects'];
  const steps = [];
  for (const [index, directory] of stepDirectories.entries()) {
    const sourcePath = `lessons/${directory}/README.md`;
    const guideContent = await read(sourcePath);
    steps.push({ id: `step_${index + 1}`, title: heading(guideContent, directory),
      subtitle: index === 0 ? '80 lessons and eight Study Tracker checkpoints' : 'Planned outline',
      sourcePath, guideContent, ...(index === 0 ? { levels } : {}) });
  }
  const data = { steps, entries, documents };
  const downloads = files.filter(file => /\.md$/i.test(file) || (file.startsWith('tasks/') && (/\.rs$/.test(file) || /(?:^|\/)Cargo\.(?:toml|lock)$/.test(file))));
  const course = path.join(output, 'public', 'course');
  const generated = path.join(output, 'src', 'generated');
  await fs.mkdir(generated, { recursive: true });
  await fs.mkdir(course, { recursive: true });
  // Remove only files previously recorded by this generator, never a directory tree.
  const manifestPath = path.join(course, '.generated-manifest.json');
  let previous = [];
  try { previous = JSON.parse(await fs.readFile(manifestPath, 'utf8')); } catch (error) { if (error.code !== 'ENOENT') throw error; }
  for (const old of previous) {
    if (!downloads.includes(old)) await fs.unlink(outputPath(course, old)).catch(error => { if (error.code !== 'ENOENT') throw error; });
  }
  for (const relative of downloads) {
    const destination = outputPath(course, relative);
    await fs.mkdir(path.dirname(destination), { recursive: true });
    await fs.copyFile(path.join(root, relative), destination);
  }
  await fs.writeFile(path.join(generated, 'content.json'), JSON.stringify(data));
  await fs.writeFile(manifestPath, JSON.stringify(downloads));
  return data;
}

if (process.argv[1] && path.resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  const regenerate = async () => {
    const data = await generateContent();
    console.log(`Course generated: ${data.entries.length} entries, ${data.documents.length} documents.`);
  };
  await regenerate();
  if (process.argv.includes('--watch')) {
    let timer;
    let running = false;
    let pending = false;
    const refresh = async () => {
      if (running) { pending = true; return; }
      running = true;
      try { await regenerate(); } catch (error) { console.error(error); }
      finally { running = false; if (pending) { pending = false; void refresh(); } }
    };
    watch(repository, { recursive: true }, (_event, filename) => {
      if (!filename) return;
      const relative = slash(filename.toString());
      if (relative.split('/').some(part => excluded.has(part) || part.startsWith('.'))) return;
      if (!/\.(?:md|rs)$|(?:^|\/)Cargo\.(?:toml|lock)$/i.test(relative)) return;
      clearTimeout(timer);
      timer = setTimeout(() => { void refresh(); }, 150);
    });
    console.log('Watching repository Markdown and Rust course sources.');
  }
}

