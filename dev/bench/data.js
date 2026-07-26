window.BENCHMARK_DATA = {
  "lastUpdate": 1785080320143,
  "repoUrl": "https://github.com/thomi137/swissqrust",
  "entries": {
    "Benchmark": [
      {
        "commit": {
          "author": {
            "name": "Thomas Prosser",
            "email": "tp@thomit.com"
          },
          "committer": {
            "name": "Thomas Prosser",
            "email": "tp@thomit.com"
          },
          "id": "c8304da82adc23d94f37f41c918a195013743e4c",
          "message": "added benchmark tests",
          "timestamp": "2026-07-25T10:00:35Z",
          "url": "https://github.com/thomi137/swissqrust/commit/c8304da82adc23d94f37f41c918a195013743e4c"
        },
        "date": 1784974131235,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "render_bill_to_pdf",
            "value": 9638515.303333336,
            "unit": "ns"
          },
          {
            "name": "render_bill_to_svg",
            "value": 4818298.637272727,
            "unit": "ns"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "Thomas Prosser",
            "email": "tp@thomit.com"
          },
          "committer": {
            "name": "Thomas Prosser",
            "email": "tp@thomit.com"
          },
          "id": "d504aad0a687177747e94b8ceb440945f6bc5bf3",
          "message": "Move cli/gui/web into crates/, drop test_output, fix demo fixture\n\nConsolidates the example consumer crates under crates/ so the repo root\nreads as \"the library plus its examples\" rather than four sibling\npackage directories. Updates every relative path the move touches:\nworkspace members, each sub-crate's swiss_qrust path dependency,\ngui's include_bytes! asset paths and packager metadata, and web's\nTrunk.toml watch paths.\n\nAlso relocates the CLI's demo data/output directories into crates/cli/\n(they're only ever used by that crate), and fixes a pre-existing bug in\nthe demo fixture itself: it paired a QRR-type reference with a\nnon-QR-IBAN, which BillData::new correctly rejects. Swapped in the\nIBAN already used and verified elsewhere in this codebase's own\ndoctests as a valid QR-IBAN.\n\ntest_output/ was untracked entirely and gitignored - nothing in the\ncodebase referenced it; it was dead weight from early experimentation.\n\nVerified: full workspace build, 93 nextest + 26 doctests, cargo package\ndry-run (crates/ correctly excluded from the published tarball), and\nthe CLI example end-to-end for both the .json and .toml fixture.",
          "timestamp": "2026-07-26T01:13:04Z",
          "url": "https://github.com/thomi137/swissqrust/commit/d504aad0a687177747e94b8ceb440945f6bc5bf3"
        },
        "date": 1785028834551,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "render_bill_to_pdf",
            "value": 9399738.82,
            "unit": "ns"
          },
          {
            "name": "render_bill_to_svg",
            "value": 4879263.3945454545,
            "unit": "ns"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "Thomas Prosser",
            "email": "tp@thomit.com"
          },
          "committer": {
            "name": "Thomas Prosser",
            "email": "tp@thomit.com"
          },
          "id": "3f0d7e5ee060ad69f4bed62439a955512ac10f1c",
          "message": "I don't know why the trunk link points to an online gambling site.",
          "timestamp": "2026-07-26T01:24:08Z",
          "url": "https://github.com/thomi137/swissqrust/commit/3f0d7e5ee060ad69f4bed62439a955512ac10f1c"
        },
        "date": 1785029154376,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "render_bill_to_pdf",
            "value": 9409925.236666666,
            "unit": "ns"
          },
          {
            "name": "render_bill_to_svg",
            "value": 4809669.014545454,
            "unit": "ns"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "Thomas Prosser",
            "email": "tp@thomit.com"
          },
          "committer": {
            "name": "Thomas Prosser",
            "email": "tp@thomit.com"
          },
          "id": "8f964e364180546744d043bf34d9408cb4cc3efc",
          "message": "Remove redundant rust.yml CI workflow\n\nIt only did a plain cargo build/test on the same branches ci.yml\nalready covers with nextest + coverage - just a duplicate CI run on\nevery push. Also drops its now-dead badge from the README; the ci.yml\nbadge already reflects real test status.",
          "timestamp": "2026-07-26T08:17:36Z",
          "url": "https://github.com/thomi137/swissqrust/commit/8f964e364180546744d043bf34d9408cb4cc3efc"
        },
        "date": 1785053961096,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "render_bill_to_pdf",
            "value": 9457966.643333336,
            "unit": "ns"
          },
          {
            "name": "render_bill_to_svg",
            "value": 4773329.619090911,
            "unit": "ns"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "Thomas Prosser",
            "email": "tp@thomit.com"
          },
          "committer": {
            "name": "Thomas Prosser",
            "email": "tp@thomit.com"
          },
          "id": "ead0bd15168e12f9c627e15da7be2fc598d47f4a",
          "message": "Generate build.rs output into OUT_DIR; drop anyhow from the library\n\nbuild.rs used to write its 4 generated files (countries.rs, cross.rs,\ncorner_marks_amount.rs, corner_marks_payable_by.rs) directly into\nsrc/generated/, which made cargo publish's own verification step\nrefuse to proceed (a build script writing into the source tree looks\nindistinguishable from an unintended mutation, even though the write\nis deterministic and harmless). Writing them into OUT_DIR instead -\nCargo's actual designated scratch space for build scripts - and\nswitching src/generated/mod.rs's includes to\ninclude!(concat!(env!(\"OUT_DIR\"), ...)) fixes that properly: verified\nvia a full `cargo publish --dry-run` with verification enabled, no\n--no-verify needed. The 4 stale checked-in copies and the exclude list\nthey required are both gone.\n\nAlso removes anyhow from the library's own dependencies - it was only\never used in two public functions (parse_bill_data, create_pdf), both\nnow returning proper thiserror enums (ParseBillDataError,\nCreatePdfError) instead of an opaque, type-erased anyhow::Error,\nconsistent with every other error type in this crate. anyhow remains\nin crates/cli and crates/gui, where it's a normal, appropriate choice\nfor application code.\n\nRemoving anyhow's `use anyhow::*;` surfaced two other issues, fixed\nhere too:\n- lib.rs had its own redundant, silently-shadowed duplicate of the\n  generated country/SVG data via direct include!s, on top of the\n  `pub mod generated` + `pub use generated::*` that already exposed\n  the same items - compiling (and, since checked in, storing) the\n  entire country dataset twice for nothing.\n- `pub use serde_json::*;` was shadowing the real `Result` at crate\n  scope with serde_json's single-parameter Result<T> alias - the\n  actual cause of a compile error hit while fixing the above. Neither\n  it nor `pub use strum::*;` gate any part of this crate's real public\n  API (crates/web already depends on strum directly and imports\n  IntoEnumIterator itself), so both blanket re-exports are gone.",
          "timestamp": "2026-07-26T13:27:17Z",
          "url": "https://github.com/thomi137/swissqrust/commit/ead0bd15168e12f9c627e15da7be2fc598d47f4a"
        },
        "date": 1785072529842,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "render_bill_to_pdf",
            "value": 10665292.51,
            "unit": "ns"
          },
          {
            "name": "render_bill_to_svg",
            "value": 5078856.026999999,
            "unit": "ns"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "Thomas Prosser",
            "email": "tp@thomit.com"
          },
          "committer": {
            "name": "Thomas Prosser",
            "email": "tp@thomit.com"
          },
          "id": "2412c6deeec8f65b659d64e687048e78bcf68c62",
          "message": "Add Open Graph/Twitter Card meta tags to the web app; fix icon path\n\nThe icon path (../assets/png/icon.png) was never updated when web/\nmoved to crates/web/ - trunk build failed outright since nobody had\nrebuilt it since that move. Fixed to ../../assets/png/icon.png.\n\nAlso adds og:*/twitter:* meta tags so sharing the deployed site\n(swissqrust.prosser.ch) on X/Facebook/WhatsApp shows a proper preview\ncard instead of a bare link.",
          "timestamp": "2026-07-26T15:31:02Z",
          "url": "https://github.com/thomi137/swissqrust/commit/2412c6deeec8f65b659d64e687048e78bcf68c62"
        },
        "date": 1785079985167,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "render_bill_to_pdf",
            "value": 9393946.018333338,
            "unit": "ns"
          },
          {
            "name": "render_bill_to_svg",
            "value": 4856993.409999999,
            "unit": "ns"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "Thomas Prosser",
            "email": "tp@thomit.com"
          },
          "committer": {
            "name": "Thomas Prosser",
            "email": "tp@thomit.com"
          },
          "id": "8dc527c9afa8490282235a7c667d4915bbb64808",
          "message": "Add publish = false to crates/web, matching cli and gui\n\nNothing stopped an accidental cargo publish --workspace from trying to\npush the WASM frontend crate to crates.io - it wouldn't make sense\nthere (hardcoded relative asset paths, wasm-bindgen/\nconsole_error_panic_hook deps, tied to this specific deployed site).",
          "timestamp": "2026-07-26T15:36:38Z",
          "url": "https://github.com/thomi137/swissqrust/commit/8dc527c9afa8490282235a7c667d4915bbb64808"
        },
        "date": 1785080319597,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "render_bill_to_pdf",
            "value": 9265936.403333334,
            "unit": "ns"
          },
          {
            "name": "render_bill_to_svg",
            "value": 4768674.029090909,
            "unit": "ns"
          }
        ]
      }
    ]
  }
}