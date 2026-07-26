// IBAN typing must produce the exact digits typed, at any speed. Alpine
// only masks on blur (not live), specifically to avoid the caret-scrambling
// class of bug the previous vanilla-DOM rewrite hit -- this is a guard
// against that coming back.
const { launch, setValue, assert, sleep } = require('./helpers');

const IBAN_SELECTOR = '[x-model="bill.iban"]';

async function testAtSpeed(delay) {
  const { browser, page } = await launch();

  await setValue(page, IBAN_SELECTOR, '');
  await page.focus(IBAN_SELECTOR);
  const target = 'CH4331999000001265789';
  await page.keyboard.type(target, { delay });

  const value = await page.evaluate((sel) => document.querySelector(sel)?.value, IBAN_SELECTOR);
  assert(value === target, `IBAN typed correctly at delay=${delay}ms while focused (got ${JSON.stringify(value)})`);

  await page.click('header'); // blur
  await sleep(100);
  const blurred = await page.evaluate((sel) => document.querySelector(sel)?.value, IBAN_SELECTOR);
  assert(blurred.replace(/ /g, '') === target, `IBAN masks correctly on blur (got ${JSON.stringify(blurred)})`);

  assert(!page.hadError(), `no JS errors at delay=${delay}ms`);

  await browser.close();
}

(async () => {
  for (const delay of [5, 50, 150]) {
    await testAtSpeed(delay);
  }
  console.log('PASS: typing.test.js');
})().catch((e) => {
  console.error(e);
  process.exit(1);
});
