window.BENCHMARK_DATA = {
  "lastUpdate": 1785029155214,
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
      }
    ]
  }
}