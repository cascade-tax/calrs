#!/usr/bin/env node
/**
 * Single-page peek: look at one calrs page, logged in, at a chosen theme and
 * viewport. Built for iterating on templates — including in parallel, which is
 * why every run is scoped to its own port, data dir and DB copy.
 *
 *   node tools/peek.mjs --port 4001 --path /dashboard --theme dark --width 1440 --out /tmp/a.png
 *   node tools/peek.mjs --port 4001 --path /u/alice/intro --anon --width 390 --out /tmp/b.png
 *
 * The server for a given --port is started on first use against a private copy
 * of the seeded demo DB and then left running, so subsequent peeks on the same
 * port are fast. Templates are cached per server process, so a template edit
 * needs --restart (the default is to restart, since that is what you almost
 * always want while editing).
 *
 * Seed DB: /tmp/calrs-screenshots/calrs.db (created by tools/shots.sh).
 * Login:   alice@example.com / password1234
 */

import { execFileSync, spawn } from 'node:child_process';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import puppeteer from 'puppeteer-core';

const ROOT = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');

function arg(name, fallback) {
  const i = process.argv.indexOf(`--${name}`);
  if (i !== -1 && process.argv[i + 1] && !process.argv[i + 1].startsWith('--')) {
    return process.argv[i + 1];
  }
  return fallback;
}
const flag = (name) => process.argv.includes(`--${name}`);

const PORT = parseInt(arg('port', '4001'), 10);
const PATHNAME = arg('path', '/dashboard');
const THEME = arg('theme', 'light');
const WIDTH = parseInt(arg('width', '1440'), 10);
const HEIGHT = parseInt(arg('height', '1000'), 10);
const OUT = arg('out', `/tmp/peek-${PORT}.png`);
const ANON = flag('anon');
const KEEP = flag('keep'); // do not restart a server that is already up
const FULL = !flag('viewport-only');
const SEED_DB = arg('seed-db', '/tmp/calrs-screenshots/calrs.db');
const DATA_DIR = arg('data-dir', `/tmp/calrs-peek-${PORT}`);
const CHROME =
  process.env.CALRS_SHOTS_CHROME ||
  '/Applications/Google Chrome.app/Contents/MacOS/Google Chrome';
const EMAIL = 'alice@example.com';
const PASSWORD = 'password1234';
const BASE = `http://127.0.0.1:${PORT}`;
const BIN = path.join(ROOT, 'target/debug/calrs');

const sleep = (ms) => new Promise((r) => setTimeout(r, ms));

async function isUp() {
  try {
    const r = await fetch(`${BASE}/auth/login`, { redirect: 'manual' });
    return r.status < 500;
  } catch {
    return false;
  }
}

function killPort() {
  try {
    execFileSync('pkill', ['-f', `calrs serve --port ${PORT}`], { stdio: 'ignore' });
  } catch {
    /* nothing listening */
  }
}

async function ensureServer() {
  if (KEEP && (await isUp())) return;
  killPort();
  await sleep(400);

  if (!fs.existsSync(BIN)) {
    throw new Error(`missing ${BIN} — run: cargo build`);
  }
  fs.mkdirSync(DATA_DIR, { recursive: true });
  // Fresh copy of the seeded DB each time the server restarts, so one agent's
  // clicking around never leaks into another's screenshots.
  if (fs.existsSync(SEED_DB)) {
    for (const suffix of ['', '-wal', '-shm']) {
      const src = SEED_DB + suffix;
      if (fs.existsSync(src)) fs.copyFileSync(src, path.join(DATA_DIR, 'calrs.db' + suffix));
    }
  }
  const log = fs.openSync(path.join(DATA_DIR, 'server.log'), 'a');
  const child = spawn(BIN, ['serve', '--port', String(PORT), '--data-dir', DATA_DIR], {
    cwd: ROOT,
    detached: true,
    stdio: ['ignore', log, log],
    env: { ...process.env, CALRS_BASE_URL: BASE, RUST_LOG: 'calrs=warn' },
  });
  child.unref();

  for (let i = 0; i < 80; i++) {
    if (await isUp()) return;
    await sleep(250);
  }
  throw new Error(`server did not come up on ${PORT}; see ${DATA_DIR}/server.log`);
}

async function main() {
  await ensureServer();

  const browser = await puppeteer.launch({
    executablePath: CHROME,
    headless: 'new',
    args: ['--no-sandbox', '--hide-scrollbars', '--force-device-scale-factor=2'],
    defaultViewport: { width: WIDTH, height: HEIGHT, deviceScaleFactor: 2 },
  });

  try {
    const page = await browser.newPage();
    await page.emulateTimezone('America/Los_Angeles');
    await page.emulateMediaFeatures([
      { name: 'prefers-color-scheme', value: THEME === 'dark' ? 'dark' : 'light' },
    ]);

    // Seed the theme preference before any page script runs.
    await page.goto(`${BASE}/auth/login`, { waitUntil: 'domcontentloaded' });
    await page.evaluate((t) => localStorage.setItem('calrs_theme', t), THEME);

    if (!ANON) {
      await page.type('#email', EMAIL);
      await page.type('#password', PASSWORD);
      await Promise.all([
        page.waitForNavigation({ waitUntil: 'networkidle2' }).catch(() => {}),
        page.click('button[type="submit"]'),
      ]);
    }

    const res = await page.goto(`${BASE}${PATHNAME}`, {
      waitUntil: 'networkidle2',
      timeout: 45000,
    });

    // The sync overlay can cover the page on calendar-backed views.
    await page
      .waitForFunction(
        () => {
          const el = document.getElementById('calrs-sync-loader');
          return !el || getComputedStyle(el).display === 'none';
        },
        { timeout: 20000 },
      )
      .catch(() => {});
    await page.evaluate(() => document.fonts && document.fonts.ready).catch(() => {});
    await sleep(350);

    await page.screenshot({ path: OUT, fullPage: FULL });
    console.log(`${OUT}  status=${res ? res.status() : '?'}  ${PATHNAME}  ${THEME}  ${WIDTH}px`);
  } finally {
    await browser.close();
  }
}

main().catch((e) => {
  console.error(String(e && e.message ? e.message : e));
  process.exit(1);
});
