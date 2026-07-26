// QR/SCOR reference-number feature: QR-IBANs get an auto-proposed valid QR
// reference; normal IBANs get an optional SCOR reference; switching IBAN
// kind clears/regenerates appropriately; invalid text shows an error.
const { launch, setValue, assert, sleep } = require('./helpers');

const IBAN_SELECTOR = '[x-model="bill.iban"]';
const REF_SELECTOR = '[x-model="bill.reference"]';

async function refState(page) {
  return page.evaluate((sel) => {
    const el = document.querySelector(sel);
    return {
      value: el?.value,
      placeholder: el?.placeholder,
    };
  }, REF_SELECTOR);
}

async function typeIban(page, iban) {
  await setValue(page, IBAN_SELECTOR, '');
  await page.focus(IBAN_SELECTOR);
  await page.keyboard.type(iban, { delay: 10 });
  await sleep(150);
}

(async () => {
  const { browser, page } = await launch();

  let s = await refState(page);
  assert(s.value === '', 'reference starts empty for the seeded (normal) IBAN');

  await typeIban(page, 'CH4331999000001265789');
  s = await refState(page);
  assert(s.placeholder === 'QR reference (required)', 'placeholder switches to QR mode');
  assert(/^\d{27}$/.test(s.value), `auto-proposed a 27-digit QR reference (got ${JSON.stringify(s.value)})`);

  const firstQrRef = s.value;

  await page.click('[data-testid="propose-reference"]');
  await sleep(100);
  s = await refState(page);
  assert(s.value !== firstQrRef, 'Propose button generates a fresh QR reference');

  await typeIban(page, 'CH9300762011623852957');
  s = await refState(page);
  assert(s.value === '', 'reference clears when switching to a normal IBAN');
  assert(s.placeholder === 'SCOR reference (optional)', 'placeholder switches to SCOR mode');

  await page.click('[data-testid="propose-reference"]');
  await sleep(100);
  s = await refState(page);
  assert(/^RF\d{2}\d+$/.test(s.value), `Propose generates a SCOR reference (got ${JSON.stringify(s.value)})`);

  await setValue(page, REF_SELECTOR, 'not-a-valid-ref');
  await sleep(100);
  const errorText = await page.evaluate(
    () => document.querySelector('[data-testid="reference-error"]')?.textContent,
  );
  assert(!!errorText, `invalid reference text shows an error (got ${JSON.stringify(errorText)})`);

  assert(!page.hadError(), 'no JS errors during the whole flow');

  await browser.close();
  console.log('PASS: reference.test.js');
})().catch((e) => {
  console.error(e);
  process.exit(1);
});
