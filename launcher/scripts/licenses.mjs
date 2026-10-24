#!/usr/bin/env node
// MIT. Generate redistributable notices from Cargo's locked package metadata.
import fs from 'node:fs';
import path from 'node:path';
const [input, output] = process.argv.slice(2);
if (!input || !output) throw new Error('Usage: licenses.mjs metadata.json output.md');
const metadata = JSON.parse(fs.readFileSync(input, 'utf8'));
const root = metadata.packages.find(p => p.name === 'bible-rs').id;
const nodes = new Map(metadata.resolve.nodes.map(n => [n.id, n]));
const included = new Set();
function visit(id) {
  if (included.has(id)) return;
  included.add(id);
  for (const dep of nodes.get(id)?.deps ?? []) {
    if (dep.dep_kinds.some(k => k.kind !== 'dev')) visit(dep.pkg);
  }
}
visit(root);
let text = '# Third-party dependency notices\n\nGenerated from the locked Cargo dependency graph for ARM64 Linux. Includes normal and build dependencies. Original project and bundled assets are credited separately in CREDITS.md.\n';
const missing = [];
const noticeDir = path.resolve(path.dirname(new URL(import.meta.url).pathname), '../licenses');
const eguiNames = new Set(['ecolor','eframe','egui','egui_glow','egui-wgpu','egui-winit','emath','epaint','epaint_default_fonts']);
for (const pkg of metadata.packages.filter(p => p.source && included.has(p.id)).sort((a,b) => a.name.localeCompare(b.name))) {
  const dir = path.dirname(pkg.manifest_path);
  const files = [];
  function scan(folder, depth = 0) {
    for (const entry of fs.readdirSync(folder, {withFileTypes: true})) {
      if (!/^(licen[cs]e|copying|notice)/i.test(entry.name)) continue;
      const filename = path.join(folder, entry.name);
      if (entry.isFile()) files.push(filename);
      else if (entry.isDirectory() && depth < 2) scan(filename, depth + 1);
    }
  }
  scan(dir);
  // These workspace crates omit their repository-level license from the crate archive.
  // Copies from the crate's exact upstream VCS revision are retained beside this script.
  if (!files.length) {
    const name = eguiNames.has(pkg.name) ? 'egui-Apache-2.0.txt'
      : pkg.name === 'accesskit_winit' ? 'accesskit-winit-Apache-2.0.txt'
      : pkg.name.startsWith('accesskit') ? 'accesskit-Apache-2.0.txt'
      : pkg.name === 'gl_generator' ? 'gl-rs-LICENSE.txt'
      : pkg.name === 'khronos_api' ? 'khronos-api-LICENSE.txt'
      : pkg.name === 'profiling' ? 'profiling-Apache-2.0.txt'
      : pkg.name === 'spirv' ? 'spirv-LICENSE.txt' : null;
    if (name) files.push(path.join(noticeDir,name));
  }
  if (pkg.name === 'epaint_default_fonts') {
    for (const name of fs.readdirSync(path.join(dir,'fonts'))) {
      if (name.endsWith('.txt')) files.push(path.join(dir,'fonts',name));
    }
  }
  if (pkg.license_file) {
    const filename = path.resolve(dir, pkg.license_file);
    if (fs.existsSync(filename) && !files.includes(filename)) files.push(filename);
  }
  if (!pkg.license || !files.length) missing.push(`${pkg.name} ${pkg.version}: ${pkg.license ?? 'no license expression'}, ${files.length} notice files`);
  text += `\n## ${pkg.name} ${pkg.version}\n\nDeclared license: ${pkg.license ?? 'see license file'}\n\nAuthors: ${pkg.authors.join(', ')}\n\nSource: https://crates.io/crates/${pkg.name}/${pkg.version}\n`;
  for (const file of files.sort()) text += `\n### ${path.basename(file)}\n\n~~~~text\n${fs.readFileSync(file,'utf8')}\n~~~~\n`;
}
fs.writeFileSync(output,text);
if (missing.length) {
  fs.writeFileSync(`${output}.review.txt`, missing.join('\n')+'\n');
  process.stderr.write('Dependency notices require review:\n'+missing.join('\n')+'\n');
  process.exitCode = 1;
}
