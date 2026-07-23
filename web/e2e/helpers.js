const puppeteer = require('puppeteer-core');

const CHROME_PATH = process.env.CHROME_PATH
  || '/Applications/Google Chrome.app/Contents/MacOS/Google Chrome';
const APP_URL = process.env.APP_URL || 'http://localhost:8080';

/**
 * Launches headless Chrome and returns { browser, page }. Attaches
 * listeners that print any JS exception or console.error to stdout, and
 * exposes `page.hadError()` to check afterwards.
 */
async function launch() {
  const browser = await puppeteer.launch({
    executablePath: CHROME_PATH,
    headless: 'new',
    args: ['--no-sandbox'],
  });
  const page = await browser.newPage();

  let hadError = false;
  page.on('pageerror', (err) => {
    hadError = true;
    console.log('PAGEERROR:', err.message);
  });
  page.on('console', (msg) => {
    if (msg.type() === 'error') {
      hadError = true;
      console.log('CONSOLE ERROR:', msg.text());
    }
  });
  page.hadError = () => hadError;

  await page.goto(APP_URL, { waitUntil: 'networkidle0' });
  // Alpine's x-init="init()" awaits the wasm module and does the first
  // renderPreview() call; wait for that to actually produce SVG content
  // rather than a fixed sleep, so tests aren't racing wasm instantiation.
  await page.waitForFunction(
    () => document.querySelector('[x-html="previewSvg"]')?.innerHTML.includes('<svg'),
    { timeout: 10000 },
  );

  return { browser, page };
}

function sleep(ms) {
  return new Promise((r) => setTimeout(r, ms));
}

/** Sets an input/textarea's value directly and fires a bubbling 'input' event. */
async function setValue(page, selector, value) {
  await page.evaluate(
    (selector, value) => {
      const el = document.querySelector(selector);
      el.value = value;
      el.dispatchEvent(new Event('input', { bubbles: true }));
    },
    selector,
    value,
  );
}

function assert(condition, message) {
  if (!condition) {
    throw new Error(`ASSERTION FAILED: ${message}`);
  }
  console.log(`ok - ${message}`);
}

module.exports = { launch, sleep, setValue, assert };
