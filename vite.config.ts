import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';
// The generator remains JavaScript so it can also run before the app is built.
// @ts-expect-error JavaScript generator has no declaration file.
import { generateContent } from './scripts/generate-content.mjs';
export default defineConfig({ server: { watch: { ignored: ['**/target/**', '**/.git/**'] } }, plugins: [react(), {
  name: 'course-content',
  configureServer(server) {
    server.watcher.add('.');
    let timer: ReturnType<typeof setTimeout>;
    let running = false;
    let pending = false;
    const regenerate = async () => {
      if (running) { pending = true; return; }
      running = true;
      try { await generateContent(); server.ws.send({ type: 'full-reload' }); }
      catch (error) { server.config.logger.error(`Course generation failed: ${String(error)}`); }
      finally { running = false; if (pending) { pending = false; void regenerate(); } }
    };
    const refresh = (path: string) => {
      if (!/(?:^|\/)(?:learning-portal|node_modules|target|\.git|dist|public|legacy)(?:\/|$)/.test(path.replaceAll('\\', '/')) && /\.(md|rs)$|Cargo\.(toml|lock)$/.test(path)) {
        clearTimeout(timer);
        timer = setTimeout(() => { void regenerate(); }, 150);
      }
    };
    server.watcher.on('change', refresh).on('add', refresh).on('unlink', refresh);
    server.httpServer?.on('close', () => clearTimeout(timer));
  },
}], base: './' });
