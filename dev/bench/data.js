window.BENCHMARK_DATA = {
  "lastUpdate": 1786018378390,
  "repoUrl": "https://github.com/mahesh-diwan/flexfetch",
  "entries": {
    "Benchmark": [
      {
        "commit": {
          "author": {
            "email": "diwanmahesh11@gmail.com",
            "name": "Mahesh Diwan",
            "username": "mahesh-diwan"
          },
          "committer": {
            "email": "diwanmahesh11@gmail.com",
            "name": "Mahesh Diwan",
            "username": "mahesh-diwan"
          },
          "distinct": true,
          "id": "4fd2077fc2a54dc2e17f4fd713458966a57e33d3",
          "message": "fix(ci): benchmark — init gh-pages branch + best-effort chart publish\n\nThe Publish step ran git fetch origin gh-pages:gh-pages before pushing, which aborts on a fresh repo where the branch doesn't exist ('couldn't find remote ref gh-pages') — reddening the Benchmark job on every main push. Added an idempotent 'Ensure gh-pages branch exists' step (creates the orphan branch once) and marked the publish step continue-on-error: the chart data push is best-effort — the binary-size gate in the 'Track binary size' step is the real regression check.",
          "timestamp": "2026-08-06T17:39:54+05:30",
          "tree_id": "086291d35706311ba43a88972b28393c7a7cf0fd",
          "url": "https://github.com/mahesh-diwan/flexfetch/commit/4fd2077fc2a54dc2e17f4fd713458966a57e33d3"
        },
        "date": 1786018378131,
        "tool": "cargo",
        "benches": [
          {
            "name": "cold_start_minimal",
            "value": 11279855,
            "range": "± 182468",
            "unit": "ns/iter"
          },
          {
            "name": "cold_start_default_pipe",
            "value": 11210666,
            "range": "± 109350",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}