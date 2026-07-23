// Broad feature smoke test: one pass touching every major interaction
// (typing with special characters, PLZ autofill, debtor toggle, country
// picker, amount, language switch, PDF download). Not exhaustive per
// feature (see typing.test.js / reference.test.js for those), just enough
// to catch a broken build across the whole app.
const path = require('path');
const fs = require('fs');
const { launch, setValue, assert, sleep } = require('./helpers');

const DOWNLOAD_DIR = path.join(__dirname, '.downloads');

const sel = {
  creditorName: '[x-model="bill.creditor_address.name"]',
  creditorPlz: '[x-model="bill.creditor_address.plz"]',
  creditorCity: '[x-model="bill.creditor_address.city"]',
  debtorToggle: '[x-model="hasDebtor"]',
  debtorName: '[x-model="bill.debtor_address.name"]',
  debtorPlz: '[x-model="bill.debtor_address.plz"]',
  debtorCity: '[x-model="bill.debtor_address.city"]',
  amount: '[x-model="bill.amount"]',
  status: '[data-testid="status"]',
};

(async () => {
  const { browser, page } = await launch();

  await setValue(page, sel.creditorName, '');
  await page.focus(sel.creditorName);
  await page.keyboard.type('Muster AG & Söhne "Import/Export"', { delay: 10 });
  let name = await page.evaluate((s) => document.querySelector(s)?.value, sel.creditorName);
  assert(name === 'Muster AG & Söhne "Import/Export"', 'special characters typed correctly');

  await setValue(page, sel.creditorPlz, '');
  await setValue(page, sel.creditorCity, '');
  await page.focus(sel.creditorPlz);
  await page.keyboard.type('8001', { delay: 30 });
  await sleep(1500);
  let city = await page.evaluate((s) => document.querySelector(s)?.value, sel.creditorCity);
  assert(city === 'Zürich', `PLZ autofill works (got ${JSON.stringify(city)})`);

  await page.evaluate((s) => {
    const cb = document.querySelector(s);
    cb.checked = true;
    cb.dispatchEvent(new Event('change', { bubbles: true }));
  }, sel.debtorToggle);
  await sleep(200);
  await page.focus(sel.debtorName);
  await page.keyboard.type('Debtor Corp', { delay: 10 });
  let debtorName = await page.evaluate((s) => document.querySelector(s)?.value, sel.debtorName);
  assert(debtorName === 'Debtor Corp', 'debtor toggle + typing works');
  await page.focus(sel.debtorPlz);
  await page.keyboard.type('3000', { delay: 10 });
  await page.focus(sel.debtorCity);
  await page.keyboard.type('Bern', { delay: 10 });

  await page.click('[data-testid="creditor-country-toggle"]');
  await sleep(100);
  await page.click('[data-testid="creditor-country-option"][data-country="DE"]');
  let country = await page.evaluate(
    () => document.querySelector('[data-testid="creditor-country-toggle"] span')?.textContent,
  );
  assert(country === 'DE', 'country picker works');

  await page.focus(sel.amount);
  await page.keyboard.type('1234.56', { delay: 10 });
  let amount = await page.evaluate((s) => document.querySelector(s)?.value, sel.amount);
  assert(amount === '1234.56', `amount field keeps digit order (got ${JSON.stringify(amount)})`);

  await page.click('[data-testid="lang-Fr"]');
  await sleep(100);
  let label = await page.evaluate(() => document.querySelector('h3')?.textContent);
  assert(label && label.length > 0, `language switch updates labels (got ${JSON.stringify(label)})`);

  fs.mkdirSync(DOWNLOAD_DIR, { recursive: true });
  const client = await page.createCDPSession();
  await client.send('Page.setDownloadBehavior', { behavior: 'allow', downloadPath: DOWNLOAD_DIR });
  await page.click('[data-testid="download"]');
  await sleep(1500);
  const status = await page.evaluate((s) => document.querySelector(s)?.textContent, sel.status);
  assert(status === 'PDF Downloaded!', `PDF download reports success (got ${JSON.stringify(status)})`);
  const pdfPath = path.join(DOWNLOAD_DIR, 'Swiss-QR-Bill.pdf');
  assert(fs.existsSync(pdfPath) && fs.statSync(pdfPath).size > 1000, 'a real PDF file was written');

  assert(!page.hadError(), 'no JS errors during the whole smoke pass');

  await browser.close();
  console.log('PASS: smoke.test.js');
})().catch((e) => {
  console.error(e);
  process.exit(1);
});
