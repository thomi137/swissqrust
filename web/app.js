/*
 * Copyright (c) 2026 Thomas Prosser
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */

// Alpine.js UI logic. All business logic (validation, checksums, the live
// SVG preview, PDF generation) is delegated to the wasm-bindgen functions in
// web/src/api.rs, exposed by Trunk as `window.wasmBindings` once the wasm
// module has loaded (see waitForWasm below). This file only owns UI state
// and browser APIs (DOM, fetch, localStorage, file download).

function waitForWasm() {
  return new Promise((resolve) => {
    if (window.wasmBindings) {
      resolve(window.wasmBindings);
      return;
    }
    window.addEventListener('TrunkApplicationStarted', () => resolve(window.wasmBindings), { once: true });
  });
}

function emptyAddress() {
  return { name: '', street: '', house_num: '', plz: '', city: '', country: 'CH' };
}

function maskIban(raw) {
  const digits = raw.replace(/\s/g, '').toUpperCase();
  return digits.match(/.{1,4}/g)?.join(' ') ?? '';
}

function todayYYYYMMDD() {
  const d = new Date();
  return `${d.getFullYear()}${String(d.getMonth() + 1).padStart(2, '0')}${String(d.getDate()).padStart(2, '0')}`;
}

// Daily running number for proposed references (the common "date + two
// digit sequence" bank convention), persisted across bills generated the
// same day, reset on a new day.
function nextDailySequence() {
  const key = 'swiss_qrust_reference_sequence';
  const today = todayYYYYMMDD();
  const stored = localStorage.getItem(key);
  let n = 1;
  if (stored) {
    const [date, num] = stored.split(':');
    if (date === today) {
      n = (parseInt(num, 10) || 0) + 1;
      if (n > 99) n = 1;
    }
  }
  localStorage.setItem(key, `${today}:${n}`);
  return n;
}

async function fetchCityByPlz(country, plz) {
  if (plz.length !== 4) return null;
  const path = country === 'CH' ? 'ch' : country === 'LI' ? 'li' : null;
  if (!path) return null;
  try {
    const res = await fetch(`https://openplzapi.org/${path}/Localities?postalCode=${plz}`);
    if (!res.ok) return null;
    const json = await res.json();
    return json[0]?.name ?? null;
  } catch {
    return null;
  }
}

function triggerDownload(bytes, filename) {
  const blob = new Blob([bytes], { type: 'application/pdf' });
  const url = URL.createObjectURL(blob);
  const link = document.createElement('a');
  link.href = url;
  link.download = filename;
  link.click();
  URL.revokeObjectURL(url);
}

function app() {
  return {
    wasm: null,
    lang: 'De',
    hasDebtor: false,
    hasSwico: false,
    status: 'Ready',
    previewSvg: '',
    labels: {},
    countries: [],
    countryDropdown: {
      creditor: { open: false, search: '' },
      debtor: { open: false, search: '' },
    },
    // Last city value auto-filled from a PLZ lookup, per address kind - lets
    // onPlzChanged tell "user typed their own city" apart from "still holds
    // what we last looked up" without storing that flag on the address
    // object itself (which gets spread verbatim into the wasm JSON payload).
    autoFilledCity: { creditor: 'Biel', debtor: null },
    referenceError: null,
    bill: {
      iban: 'CH93 0076 2011 6238 5295 7',
      creditor_address: {
        name: 'Robert Schneider AG',
        street: '',
        house_num: '',
        plz: '2501',
        city: 'Biel',
        country: 'CH',
      },
      debtor_address: emptyAddress(),
      currency: 'CHF',
      amount: '',
      reference: '',
      unstructured_message: '',
      bill_information: null,
      alternative_schemes: null,
    },
    swico: {
      invoice_number: '',
      invoice_date: '',
      customer_reference: '',
      vat_rate: '',
      discount_percent: '',
      discount_days: '',
    },

    async init() {
      this.wasm = await waitForWasm();
      this.loadLabels();
      this.loadCountries();
      this.renderPreview();

      this.$watch('lang', () => {
        this.loadLabels();
        this.loadCountries();
      });

      this.$watch('bill.iban', () => this.onIbanChanged());
      this.$watch('bill.reference', () => this.validateReference());
      for (const kind of ['creditor', 'debtor']) {
        this.$watch(`bill.${kind}_address.plz`, () => this.onPlzChanged(kind));
      }

      // Re-render the live preview whenever any bill field changes. Errors
      // (e.g. an incomplete IBAN mid-typing) just leave the last successful
      // preview on screen rather than blanking it out.
      this.$watch('bill', () => this.renderPreview(), { deep: true });
      this.$watch('swico', () => this.renderPreview(), { deep: true });
      this.$watch('lang', () => this.renderPreview());
      this.$watch('hasDebtor', () => this.renderPreview());
      this.$watch('hasSwico', () => this.renderPreview());
    },

    loadLabels() {
      this.labels = JSON.parse(this.wasm.labels_json(this.lang));
      this.status = this.labels.StatusReady;
    },

    loadCountries() {
      this.countries = JSON.parse(this.wasm.country_list_json(this.lang));
    },

    filteredCountries(kind) {
      const search = this.countryDropdown[kind].search.toLowerCase();
      if (!search) return this.countries;
      return this.countries.filter(
        (c) => c.code.toLowerCase().includes(search) || c.name.toLowerCase().includes(search),
      );
    },

    countryLabel(code) {
      return code;
    },

    selectCountry(kind, code) {
      this.bill[`${kind}_address`].country = code;
      this.countryDropdown[kind].open = false;
      this.countryDropdown[kind].search = '';
    },

    onIbanFocus() {
      this.bill.iban = this.bill.iban.replace(/\s/g, '');
    },

    onIbanBlur() {
      this.bill.iban = maskIban(this.bill.iban);
    },

    get ibanCleaned() {
      return this.bill.iban.replace(/\s/g, '').toUpperCase();
    },

    get ibanValid() {
      return !!this.wasm && this.ibanCleaned.length > 0 && this.wasm.is_valid_iban_js(this.ibanCleaned);
    },

    get isQrIban() {
      return !!this.wasm && this.wasm.is_qr_iban_js(this.ibanCleaned);
    },

    onIbanChanged() {
      const isQr = this.isQrIban;
      const hasValidQrRef = isQr && JSON.parse(this.wasm.validate_reference_json(this.bill.reference, true)).valid;
      if (isQr && !hasValidQrRef) {
        this.proposeReference();
      } else if (!isQr && /^\d{27}$/.test(this.bill.reference.trim())) {
        this.bill.reference = '';
      }
    },

    validateReference() {
      const result = JSON.parse(this.wasm.validate_reference_json(this.bill.reference, this.isQrIban));
      this.referenceError = result.error;
    },

    proposeReference() {
      const raw = `${todayYYYYMMDD()}${String(nextDailySequence()).padStart(2, '0')}`;
      this.bill.reference = this.isQrIban
        ? this.wasm.generate_qr_reference(raw)
        : this.wasm.generate_scor_reference(raw);
    },

    async onPlzChanged(kind) {
      const addr = this.bill[`${kind}_address`];
      if (addr.plz.length !== 4) return;
      // Don't clobber a city the user typed themselves - only skip the
      // lookup when the current value is still what we last auto-filled.
      if (addr.city && addr.city !== this.autoFilledCity[kind]) return;
      const city = await fetchCityByPlz(addr.country, addr.plz);
      if (city) {
        addr.city = city;
        this.autoFilledCity[kind] = city;
      }
    },

    buildSwicoBillInformation() {
      if (!this.hasSwico) return null;
      const s = this.swico;
      const payment_conditions =
        s.discount_percent.trim() !== '' && s.discount_days.trim() !== ''
          ? [{ discount: parseFloat(s.discount_percent), days: parseInt(s.discount_days, 10) }]
          : null;
      return {
        invoice_number: s.invoice_number.trim() === '' ? null : s.invoice_number.trim(),
        invoice_date: s.invoice_date === '' ? null : s.invoice_date,
        customer_reference: s.customer_reference.trim() === '' ? null : s.customer_reference.trim(),
        vat_rate: s.vat_rate.trim() === '' ? null : parseFloat(s.vat_rate),
        payment_conditions,
      };
    },

    buildInputBillJson() {
      const bill = {
        ...this.bill,
        iban: this.ibanCleaned,
        debtor_address: this.hasDebtor ? this.bill.debtor_address : null,
        amount: this.bill.amount.trim() === '' ? null : this.bill.amount,
        reference: this.bill.reference.trim() === '' ? null : this.bill.reference,
        unstructured_message: this.bill.unstructured_message === '' ? null : this.bill.unstructured_message,
        swico_bill_information: this.buildSwicoBillInformation(),
      };
      return JSON.stringify(bill);
    },

    renderPreview() {
      try {
        this.previewSvg = this.wasm.render_preview_svg(this.buildInputBillJson(), this.lang);
      } catch {
        // Keep showing the last successful preview.
      }
    },

    download() {
      try {
        const bytes = this.wasm.render_pdf(this.buildInputBillJson(), this.lang);
        triggerDownload(bytes, 'Swiss-QR-Bill.pdf');
        this.status = this.labels.StatusDownloaded;
      } catch (e) {
        this.status = `Error: ${e}`;
      }
    },
  };
}

window.app = app;
