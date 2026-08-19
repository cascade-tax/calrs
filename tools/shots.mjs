#!/usr/bin/env node
/**
 * calrs screenshot harness.
 *
 * Drives the locally-installed Google Chrome via puppeteer-core and captures
 * full-page screenshots of the whole web UI at several viewport/theme
 * combinations.
 *
 * It assumes a calrs server is already running against the seeded demo DB
 * (see tools/shots.sh, which does the build/seed/serve orchestration).
 *
 * Usage:
 *   node tools/shots.mjs [--base http://127.0.0.1:3999] [--db /tmp/calrs-screenshots/calrs.db]
 *                        [--outdir /tmp/calrs-shots] [--only <substring>]
 *                        [--combos desktop-light,mobile-dark]
 */

import { execFileSync } from 'node:child_process';
import fs from 'node:fs';
import path from 'node:path';
import puppeteer from 'puppeteer-core';

// ── CLI args ──────────────────────────────────────────────────────────
function arg(name, fallback) {
  const i = process.argv.indexOf(`--${name}`);
  if (i !== -1 && process.argv[i + 1] && !process.argv[i + 1].startsWith('--')) {
    return process.argv[i + 1];
  }
  return fallback;
}

const BASE = (arg('base', process.env.CALRS_SHOTS_BASE || 'http://127.0.0.1:3999')).replace(/\/$/, '');
const DB = arg('db', process.env.CALRS_SHOTS_DB || '/tmp/calrs-screenshots/calrs.db');
const OUTDIR = arg('outdir', process.env.CALRS_SHOTS_OUTDIR || '/tmp/calrs-shots');
const ONLY = arg('only', null);
const CHROME =
  process.env.CALRS_SHOTS_CHROME ||
  '/Applications/Google Chrome.app/Contents/MacOS/Google Chrome';
const EMAIL = process.env.CALRS_SHOTS_EMAIL || 'alice@example.com';
const PASSWORD = process.env.CALRS_SHOTS_PASSWORD || 'password1234';
const TIMEZONE = arg('tz', process.env.CALRS_SHOTS_TZ || 'Europe/Paris');

const ALL_COMBOS = [
  { key: 'desktop-light', viewport: 'desktop', theme: 'light' },
  { key: 'desktop-dark', viewport: 'desktop', theme: 'dark' },
  { key: 'mobile-light', viewport: 'mobile', theme: 'light' },
  { key: 'mobile-dark', viewport: 'mobile', theme: 'dark' },
];
const comboFilter = arg('combos', null);
const COMBOS = comboFilter
  ? ALL_COMBOS.filter((c) => comboFilter.split(',').map((s) => s.trim()).includes(c.key))
  : ALL_COMBOS;

const VIEWPORTS = {
  desktop: { width: 1440, height: 900, deviceScaleFactor: 2, isMobile: false, hasTouch: false },
  mobile: { width: 390, height: 844, deviceScaleFactor: 2, isMobile: true, hasTouch: true },
};

// ── DB discovery (python3 sqlite3 — no node sqlite dependency) ────────
function query(sql) {
  const py = `
import sqlite3, sys, json
con = sqlite3.connect("file:" + sys.argv[1] + "?mode=ro", uri=True)
try:
    rows = con.execute(sys.argv[2]).fetchall()
except Exception as e:
    print(json.dumps({"error": str(e)}))
    sys.exit(0)
print(json.dumps({"rows": rows}))
`;
  const out = execFileSync('python3', ['-c', py, DB, sql], { encoding: 'utf8' });
  const parsed = JSON.parse(out);
  if (parsed.error) {
    console.warn(`  ! DB query failed (${sql}): ${parsed.error}`);
    return [];
  }
  return parsed.rows;
}

function discover() {
  if (!fs.existsSync(DB)) {
    console.warn(`! DB not found at ${DB} — dynamic routes will be skipped.`);
    return {};
  }
  const et = query(
    `SELECT et.slug, et.id FROM event_types et
     JOIN accounts a ON a.id = et.account_id
     JOIN users u ON u.id = a.user_id
     WHERE u.username = 'alice' AND et.team_id IS NULL
     ORDER BY (et.slug = 'intro') DESC, et.enabled DESC, et.created_at
     LIMIT 1`
  );
  const team = query(`SELECT slug FROM teams WHERE slug IS NOT NULL ORDER BY created_at LIMIT 1`);
  return {
    eventTypeSlug: et.length ? et[0][0] : null,
    eventTypeId: et.length ? et[0][1] : null,
    teamSlug: team.length ? team[0][0] : null,
  };
}

// ── Route table ───────────────────────────────────────────────────────
function buildRoutes(d) {
  const routes = [
    { name: '01-dashboard-overview', path: '/dashboard', auth: true },
    { name: '02-dashboard-event-types', path: '/dashboard/event-types', auth: true },
    { name: '03-dashboard-bookings', path: '/dashboard/bookings', auth: true },
    { name: '04-dashboard-sources', path: '/dashboard/sources', auth: true },
    { name: '05-dashboard-teams', path: '/dashboard/teams', auth: true },
    { name: '06-dashboard-invite-links', path: '/dashboard/invite-links', auth: true },
    { name: '07-dashboard-settings', path: '/dashboard/settings', auth: true },
    { name: '08-dashboard-admin', path: '/dashboard/admin', auth: true },
  ];

  if (d.eventTypeSlug) {
    routes.push({
      name: '09-event-type-edit',
      path: `/dashboard/event-types/${encodeURIComponent(d.eventTypeSlug)}/edit`,
      auth: true,
    });
  } else {
    routes.push({ name: '09-event-type-edit', skip: 'no personal event type found in DB' });
  }

  // NOTE: the real route is /dashboard/troubleshoot?event_type=<id> (query param),
  // not /dashboard/troubleshoot/<id>. See `troubleshoot()` in src/web/mod.rs.
  if (d.eventTypeId) {
    routes.push({
      name: '10-troubleshoot',
      path: `/dashboard/troubleshoot?event_type=${encodeURIComponent(d.eventTypeId)}`,
      auth: true,
    });
  } else {
    routes.push({ name: '10-troubleshoot', skip: 'no personal event type found in DB' });
  }

  if (d.eventTypeSlug) {
    routes.push({
      name: '11-event-type-overrides',
      path: `/dashboard/event-types/${encodeURIComponent(d.eventTypeSlug)}/overrides`,
      auth: true,
    });
  } else {
    routes.push({ name: '11-event-type-overrides', skip: 'no personal event type found in DB' });
  }

  routes.push(
    { name: '12-auth-login', path: '/auth/login', auth: false },
    { name: '13-auth-register', path: '/auth/register', auth: false },
    { name: '14-public-profile-alice', path: '/u/alice', auth: false }
  );

  if (d.eventTypeSlug) {
    routes.push({
      name: '15-slot-picker',
      path: `/u/alice/${encodeURIComponent(d.eventTypeSlug)}`,
      auth: false,
      slots: true,
    });
    routes.push({
      name: '16-booking-form',
      path: `/u/alice/${encodeURIComponent(d.eventTypeSlug)}`,
      auth: false,
      slots: true,
      clickFirstSlot: true,
    });
  } else {
    routes.push({ name: '15-slot-picker', skip: 'no personal event type found in DB' });
    routes.push({ name: '16-booking-form', skip: 'no personal event type found in DB' });
  }

  if (d.teamSlug) {
    routes.push({ name: '17-team-profile', path: `/team/${encodeURIComponent(d.teamSlug)}`, auth: false });
  } else {
    routes.push({ name: '17-team-profile', skip: 'no team found in DB' });
  }

  return routes;
}

// ── Helpers ───────────────────────────────────────────────────────────
const sleep = (ms) => new Promise((r) => setTimeout(r, ms));

async function newPage(context, combo) {
  const page = await context.newPage();
  await page.setViewport(VIEWPORTS[combo.viewport]);
  // Pin the clock so guest-facing times line up with the seeded host timezone
  // instead of whatever the reviewer's laptop is set to.
  await page.emulateTimezone(TIMEZONE).catch(() => {});
  await page.emulateMediaFeatures([
    { name: 'prefers-color-scheme', value: combo.theme },
    { name: 'prefers-reduced-motion', value: 'reduce' },
  ]);
  // Seed the theme before any page script runs so base.html's inline
  // <head> bootstrap picks it up and there is no flash / wrong theme.
  await page.evaluateOnNewDocument((theme) => {
    try {
      localStorage.setItem('calrs_theme', theme);
    } catch (e) {
      /* storage unavailable */
    }
  }, combo.theme);
  page.setDefaultNavigationTimeout(45000);
  page.setDefaultTimeout(45000);
  return page;
}

async function settle(page) {
  // Wait for the global sync overlay (base.html #calrs-sync-loader) to be hidden.
  try {
    await page.waitForFunction(
      () => {
        const el = document.getElementById('calrs-sync-loader');
        if (!el) return true;
        const s = window.getComputedStyle(el);
        return s.display === 'none' || s.visibility === 'hidden' || el.offsetParent === null;
      },
      { timeout: 30000, polling: 200 }
    );
  } catch (e) {
    /* leave it visible and capture anyway */
  }
  // Let fonts, images and any deferred slot fetch land.
  try {
    await page.evaluate(() => document.fonts && document.fonts.ready);
  } catch (e) {
    /* ignore */
  }
  await sleep(600);
}

async function login(page) {
  const resp = await page.goto(`${BASE}/auth/login`, { waitUntil: 'networkidle2' });
  if (!resp || !resp.ok()) throw new Error(`login page returned ${resp ? resp.status() : 'no response'}`);
  await page.waitForSelector('input[name="email"]');
  await page.type('input[name="email"]', EMAIL);
  await page.type('input[name="password"]', PASSWORD);
  await Promise.all([
    page.waitForNavigation({ waitUntil: 'networkidle2' }).catch(() => {}),
    page.click('form[action="/auth/login"] button[type="submit"]'),
  ]);
  const url = page.url();
  if (/\/auth\/login/.test(url)) {
    throw new Error(`login failed, still on ${url}`);
  }
  return url;
}

// ── Main ──────────────────────────────────────────────────────────────
const results = [];
const written = [];
const warnings = new Map();

async function capture(page, route, combo) {
  const file = path.join(OUTDIR, `${route.name}__${combo.key}.png`);
  const label = `${route.name} [${combo.key}]`;
  const record = { name: route.name, combo: combo.key, file, ok: true, note: '' };

  // Sub-resource failures worth reporting. /logo and the avatar endpoints
  // 404 by design when nothing has been uploaded (templates fall back), so
  // they are not signal.
  const BENIGN_404 = [/\/logo$/, /\/avatar\//, /\/team-avatar\//];
  const assetErrors = new Set();
  const onResponse = (res) => {
    if (res.status() < 400) return;
    const url = res.url();
    if (res.request().isNavigationRequest()) return;
    if (BENIGN_404.some((re) => re.test(url))) return;
    assetErrors.add(`${res.status()} ${url.replace(BASE, '')}`);
  };
  const consoleErrors = [];
  const onConsole = (m) => {
    const text = m.text();
    // Redundant with the response listener above, and fires for benign 404s.
    if (m.type() !== 'error' || /Failed to load resource/.test(text)) return;
    consoleErrors.push(text.slice(0, 200));
  };
  page.on('response', onResponse);
  page.on('console', onConsole);

  try {
    const resp = await page.goto(`${BASE}${route.path}`, { waitUntil: 'networkidle2' });
    const status = resp ? resp.status() : 0;
    record.status = status;
    if (status >= 400) {
      record.ok = false;
      record.note = `HTTP ${status}`;
    }
    await settle(page);

    if (route.slots) {
      // Slots are fetched client-side; the first available day auto-selects.
      try {
        await page.waitForSelector('a.slot-pill, .week-slot-pill, .slot-empty', { timeout: 30000 });
      } catch (e) {
        record.ok = false;
        record.note = appendNote(record.note, 'no slots rendered within 30s');
      }
      await sleep(500);
    }

    if (route.clickFirstSlot) {
      const pill = await page.$('a.slot-pill');
      if (!pill) {
        record.ok = false;
        record.note = appendNote(record.note, 'no bookable slot pill to click');
      } else {
        await Promise.all([
          page.waitForNavigation({ waitUntil: 'networkidle2' }).catch(() => {}),
          pill.click(),
        ]);
        await settle(page);
        record.finalUrl = page.url();
        if (!/\/book/.test(page.url())) {
          record.ok = false;
          record.note = appendNote(record.note, `click did not reach a booking form (${page.url()})`);
        }
      }
    }

    // Cheap error-page heuristic against booking_action_error.html / 404 bodies.
    const bodyHint = await page.evaluate(() => (document.body ? document.body.innerText.slice(0, 400) : ''));
    if (/^\s*(Not Found|Internal Server Error|Method Not Allowed)\s*$/i.test(bodyHint)) {
      record.ok = false;
      record.note = appendNote(record.note, `error body: ${bodyHint.trim().slice(0, 80)}`);
    }

    await page.screenshot({ path: file, fullPage: true, captureBeyondViewport: true });
  } catch (err) {
    record.ok = false;
    record.note = appendNote(record.note, err.message);
    // Still try to capture whatever is on screen.
    try {
      await page.screenshot({ path: file, fullPage: true });
    } catch (e) {
      record.note = appendNote(record.note, `screenshot failed: ${e.message}`);
    }
  } finally {
    page.off('console', onConsole);
    page.off('response', onResponse);
  }

  // Missing sub-resources and JS errors are reported as warnings: the page
  // still rendered and the screenshot is still worth looking at.
  for (const a of assetErrors) warnings.set(a, (warnings.get(a) || 0) + 1);
  for (const e of consoleErrors) warnings.set(`js: ${e}`, (warnings.get(`js: ${e}`) || 0) + 1);
  const warnCount = assetErrors.size + consoleErrors.length;
  if (warnCount) record.warn = warnCount;

  if (fs.existsSync(file)) {
    record.bytes = fs.statSync(file).size;
    written.push(file);
    if (record.bytes < 10240) {
      record.ok = false;
      record.note = appendNote(record.note, `suspiciously small (${record.bytes} bytes)`);
    }
  } else {
    record.ok = false;
    record.bytes = 0;
    record.note = appendNote(record.note, 'no file written');
  }

  results.push(record);
  console.log(
    `  ${record.ok ? '✓' : '✗'} ${label} -> ${path.basename(file)} (${record.bytes || 0} B)` +
      (record.warn ? `  (${record.warn} warn)` : '') +
      (record.note ? `  [${record.note}]` : '')
  );
}

function appendNote(existing, extra) {
  return existing ? `${existing}; ${extra}` : extra;
}

async function main() {
  if (!fs.existsSync(CHROME)) {
    console.error(`Chrome not found at: ${CHROME}`);
    process.exit(1);
  }
  fs.mkdirSync(OUTDIR, { recursive: true });

  const discovered = discover();
  console.log(`Base URL      : ${BASE}`);
  console.log(`DB            : ${DB}`);
  console.log(`Out dir       : ${OUTDIR}`);
  console.log(`Event type    : ${discovered.eventTypeSlug || '(none)'} / ${discovered.eventTypeId || '(none)'}`);
  console.log(`Team slug     : ${discovered.teamSlug || '(none)'}`);

  let routes = buildRoutes(discovered);
  if (ONLY) routes = routes.filter((r) => r.name.includes(ONLY) || (r.path || '').includes(ONLY));
  if (!routes.length) {
    console.error(`No routes matched --only "${ONLY}"`);
    process.exit(1);
  }
  console.log(`Routes        : ${routes.length}  Combos: ${COMBOS.map((c) => c.key).join(', ')}`);
  console.log('');

  const skipped = routes.filter((r) => r.skip);
  routes = routes.filter((r) => !r.skip);

  const browser = await puppeteer.launch({
    executablePath: CHROME,
    headless: 'new',
    args: [
      '--hide-scrollbars',
      '--no-first-run',
      '--no-default-browser-check',
      '--disable-features=Translate,MediaRouter',
      '--force-color-profile=srgb',
      '--font-render-hinting=none',
    ],
  });

  try {
    for (const combo of COMBOS) {
      console.log(`── ${combo.key} ──`);
      const authRoutes = routes.filter((r) => r.auth);
      const guestRoutes = routes.filter((r) => !r.auth);

      if (authRoutes.length) {
        const ctx = await browser.createBrowserContext();
        const page = await newPage(ctx, combo);
        try {
          await login(page);
          for (const route of authRoutes) await capture(page, route, combo);
        } catch (err) {
          console.log(`  ✗ login failed for ${combo.key}: ${err.message}`);
          for (const route of authRoutes) {
            results.push({
              name: route.name,
              combo: combo.key,
              file: path.join(OUTDIR, `${route.name}__${combo.key}.png`),
              ok: false,
              bytes: 0,
              note: `login failed: ${err.message}`,
            });
          }
        } finally {
          await ctx.close();
        }
      }

      if (guestRoutes.length) {
        // Fresh context = no session cookie, so these render as a guest.
        const ctx = await browser.createBrowserContext();
        const page = await newPage(ctx, combo);
        for (const route of guestRoutes) await capture(page, route, combo);
        await ctx.close();
      }
      console.log('');
    }
  } finally {
    await browser.close();
  }

  const failed = results.filter((r) => !r.ok);

  console.log('=== FILES WRITTEN ===');
  for (const f of written.sort()) console.log(f);

  console.log('');
  console.log(`=== SUMMARY: ${results.length - failed.length}/${results.length} captured cleanly ===`);
  for (const r of skipped) console.log(`SKIPPED  ${r.name}: ${r.skip}`);
  if (failed.length) {
    for (const r of failed) console.log(`PROBLEM  ${r.name} [${r.combo}]: ${r.note}`);
  } else if (!skipped.length) {
    console.log('No route failures.');
  }

  if (warnings.size) {
    console.log('');
    console.log('=== WARNINGS (page still rendered) ===');
    for (const [what, count] of [...warnings.entries()].sort((a, b) => b[1] - a[1])) {
      console.log(`${String(count).padStart(3)}x  ${what}`);
    }
  }

  process.exitCode = 0;
}

main().catch((err) => {
  console.error(err);
  process.exit(1);
});
