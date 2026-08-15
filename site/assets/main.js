/* flexfetch landing page (v5) — interactions */
(function () {
  "use strict";

  // progressive enhancement: reveals only hide when JS is active
  document.documentElement.classList.add("js");

  const prefersReduced = window.matchMedia("(prefers-reduced-motion: reduce)").matches;
  const finePointer = window.matchMedia("(pointer: fine)").matches;

  /* ---------- magnetic buttons (CSS-var driven, rAF-throttled) ---------- */
  if (finePointer && !prefersReduced) {
    document.querySelectorAll(".magnetic").forEach((el) => {
      let raf = null;
      const move = (e) => {
        if (raf) return;
        raf = requestAnimationFrame(() => {
          const r = el.getBoundingClientRect();
          const x = (e.clientX - r.left - r.width / 2) / r.width;
          const y = (e.clientY - r.top - r.height / 2) / r.height;
          el.style.setProperty("--mx", (x * 9).toFixed(2) + "px");
          el.style.setProperty("--my", (y * 7).toFixed(2) + "px");
          raf = null;
        });
      };
      const leave = () => {
        el.style.setProperty("--mx", "0px");
        el.style.setProperty("--my", "0px");
      };
      el.addEventListener("pointermove", move);
      el.addEventListener("pointerleave", leave);
    });
  }

  /* ---------- spotlight cards ---------- */
  document.querySelectorAll(".spot").forEach((card) => {
    card.addEventListener("pointermove", (e) => {
      const r = card.getBoundingClientRect();
      card.style.setProperty("--mx", e.clientX - r.left + "px");
      card.style.setProperty("--my", e.clientY - r.top + "px");
    });
  });

  /* ---------- hero terminal tilt (transform only) ---------- */
  const termWrap = document.querySelector(".term-wrap");
  if (termWrap && finePointer && !prefersReduced) {
    const term = termWrap.querySelector(".term");
    let raf = null;
    const move = (e) => {
      if (raf) return;
      raf = requestAnimationFrame(() => {
        const r = termWrap.getBoundingClientRect();
        const px = (e.clientX - r.left) / r.width - 0.5;
        const py = (e.clientY - r.top) / r.height - 0.5;
        term.style.setProperty("--ry", (px * 5).toFixed(2) + "deg");
        term.style.setProperty("--rx", (-py * 4).toFixed(2) + "deg");
        raf = null;
      });
    };
    const leave = () => {
      term.classList.add("settle");
      term.style.setProperty("--rx", "0deg");
      term.style.setProperty("--ry", "0deg");
      setTimeout(() => term.classList.remove("settle"), 500);
    };
    termWrap.addEventListener("pointermove", move);
    termWrap.addEventListener("pointerleave", leave);
  }

  /* ---------- reveal on scroll (stagger via --i) ---------- */
  const revealEls = document.querySelectorAll(".reveal");
  if ("IntersectionObserver" in window) {
    const io = new IntersectionObserver(
      (entries) => {
        entries.forEach((e) => {
          if (e.isIntersecting) {
            e.target.classList.add("in");
            io.unobserve(e.target);
          }
        });
      },
      { threshold: 0.12, rootMargin: "0px 0px -40px 0px" }
    );
    revealEls.forEach((el) => io.observe(el));
  } else {
    revealEls.forEach((el) => el.classList.add("in"));
  }

  /* ---------- toast ---------- */
  const toastEl = document.getElementById("toast");
  let toastTimer = null;
  function toast(msg) {
    if (!toastEl) return;
    toastEl.innerHTML =
      '<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polyline points="20 6 9 17 4 12"></polyline></svg>' +
      "<span>" + msg + "</span>";
    toastEl.classList.add("show");
    clearTimeout(toastTimer);
    toastTimer = setTimeout(() => toastEl.classList.remove("show"), 2200);
  }

  /* ---------- copy buttons ---------- */
  document.querySelectorAll("[data-copy]").forEach((btn) => {
    btn.addEventListener("click", () => {
      const id = btn.dataset.copy;
      const text = (document.getElementById(id) || {}).textContent || "";
      const done = () => {
        btn.classList.add("copied");
        btn.innerHTML =
          '<svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.2" stroke-linecap="round" stroke-linejoin="round"><polyline points="20 6 9 17 4 12"></polyline></svg>copied';
        clearTimeout(btn._resetTimer);
        btn._resetTimer = setTimeout(() => {
          btn.classList.remove("copied");
          btn.innerHTML =
            '<svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round"><rect x="9" y="9" width="13" height="13" rx="2"></rect><path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"></path></svg>copy';
        }, 1800);
        toast("copied to clipboard");
      };
      if (navigator.clipboard && window.isSecureContext) {
        navigator.clipboard.writeText(text).then(done).catch(() => fallbackCopy(text, done));
      } else {
        fallbackCopy(text, done);
      }
    });
  });
  function fallbackCopy(text, done) {
    const ta = document.createElement("textarea");
    ta.value = text;
    ta.style.position = "fixed";
    ta.style.opacity = "0";
    document.body.appendChild(ta);
    ta.select();
    try {
      document.execCommand("copy");
      done();
    } catch (e) {
      /* ignore */
    }
    document.body.removeChild(ta);
  }

  /* ---------- stat count-up (tabular, fires once on reveal, reduced-motion safe) ---------- */
  const counters = document.querySelectorAll("[data-count]");
  if (counters.length && "IntersectionObserver" in window) {
    const animate = (el) => {
      const target = parseFloat(el.dataset.count || "0");
      const decimals = parseInt(el.dataset.decimals || "0", 10);
      const suffix = el.dataset.suffix || "";
      const dur = 1100;
      if (prefersReduced) {
        el.textContent = el.dataset.count + suffix;
        return;
      }
      const t0 = performance.now();
      const step = (now) => {
        const p = Math.min((now - t0) / dur, 1);
        const eased = 1 - Math.pow(1 - p, 3);
        const v = target * eased;
        el.textContent =
          v.toFixed(decimals) + suffix;
        if (p < 1) requestAnimationFrame(step);
      };
      requestAnimationFrame(step);
    };
    const cio = new IntersectionObserver(
      (entries) => {
        entries.forEach((e) => {
          if (e.isIntersecting) {
            animate(e.target);
            cio.unobserve(e.target);
          }
        });
      },
      { threshold: 0.4 }
    );
    counters.forEach((el) => cio.observe(el));
  } else {
    counters.forEach((el) => (el.textContent = el.dataset.count + (el.dataset.suffix || "")));
  }

  /* ---------- hero terminal: skeleton -> render, with error state ---------- */
  const heroTerm = document.getElementById("hero-terminal");
  if (heroTerm) {
    const skel = heroTerm.querySelector(".skel");
    const err = heroTerm.querySelector(".term-err");
    const retry = err && err.querySelector(".retry");
    let loaded = false;

    const loadHero = () => {
      if (loaded) return;
      fetch("assets/hero.html")
        .then((r) => (r.ok ? r.text() : Promise.reject(new Error("http " + r.status))))
        .then((html) => {
          if (!html || !html.trim()) throw new Error("empty render");
          loaded = true;
          const wrap = document.createElement("div");
          wrap.className = "fade-in";
          // split the real output into lines and stagger each line's print-in,
          // like watching flexfetch fetch your system (one-shot, ~0.8s total)
          wrap.innerHTML = html
            .split("\n")
            .filter(Boolean)
            .map((line, i) => '<span class="hero-line" style="--n:' + i + '">' + line + "</span>")
            .join("");
          heroTerm.innerHTML = "";
          heroTerm.appendChild(wrap);
        })
        .catch(() => {
          if (skel) skel.remove();
          if (err) err.classList.add("show");
        });
    };
    if (retry) retry.addEventListener("click", loadHero);
    loadHero();
  }

  /* ---------- nav state ---------- */
  const nav = document.getElementById("nav");
  const onScroll = () => nav.classList.toggle("scrolled", window.scrollY > 8);
  window.addEventListener("scroll", onScroll, { passive: true });
  onScroll();

  /* ---------- active nav section ---------- */
  const sections = document.querySelectorAll("section[id]");
  const navLinks = document.querySelectorAll(".nav-links a");
  if ("IntersectionObserver" in window) {
    const sectionIo = new IntersectionObserver(
      (entries) => {
        entries.forEach((entry) => {
          if (entry.isIntersecting) {
            navLinks.forEach((l) => l.classList.remove("active"));
            const active = document.querySelector('.nav-links a[href="#' + entry.target.id + '"]');
            if (active) active.classList.add("active");
          }
        });
      },
      { threshold: 0.3, rootMargin: "-80px 0px -40% 0px" }
    );
    sections.forEach((s) => sectionIo.observe(s));
  }

  /* ---------- hamburger + mobile menu (staggered) ---------- */
  const hamburger = document.getElementById("hamburger");
  const navMobile = document.getElementById("nav-mobile");
  if (hamburger && navMobile) {
    function setMenu(open) {
      hamburger.classList.toggle("open", open);
      navMobile.classList.toggle("open", open);
      hamburger.setAttribute("aria-expanded", String(open));
      document.body.style.overflow = open ? "hidden" : "";
      if (open) {
        const target = navMobile.querySelector("a");
        if (target) target.focus({ preventScroll: true });
      }
    }
    hamburger.addEventListener("click", () =>
      setMenu(!hamburger.classList.contains("open"))
    );
    navMobile.querySelectorAll("a").forEach((a) =>
      a.addEventListener("click", () => setMenu(false))
    );
    document.addEventListener("keydown", (e) => {
      if (e.key === "Escape" && hamburger.classList.contains("open")) {
        setMenu(false);
        hamburger.focus();
      }
    });
  }

  /* ---------- module search (with empty state) ---------- */
  const searchInput = document.getElementById("mod-search");
  const searchCount = document.getElementById("mod-count");
  const modGrid = document.getElementById("mods-grid");
  const modsEmpty = document.getElementById("mods-empty");
  if (searchInput && modGrid) {
    const mods = modGrid.querySelectorAll(".mod");
    const moduleCount = [...mods].filter((m) => m.dataset.tag !== "layout").length;
    if (searchCount) searchCount.textContent = moduleCount + " modules";
    // The grid shows a trimmed set by default; searching or expanding
    // reveals every module (all stay in the DOM so search can find them).
    const modsAllBtn = document.getElementById("mods-all");
    const syncModsAll = () => {
      if (!modsAllBtn) return;
      const expanded = modGrid.classList.contains("show-all");
      modsAllBtn.setAttribute("aria-expanded", String(expanded));
      modsAllBtn.childNodes[0].textContent = expanded
        ? "Show fewer"
        : "Show all 38 modules";
    };
    if (modsAllBtn) {
      modsAllBtn.addEventListener("click", () => {
        modGrid.classList.toggle("show-all");
        syncModsAll();
      });
    }
    searchInput.addEventListener("input", () => {
      const q = searchInput.value.toLowerCase().trim();
      modGrid.classList.toggle("filtered", q !== "");
      syncModsAll();
      let shown = 0;
      mods.forEach((m) => {
        const name = (m.dataset.name || "").toLowerCase();
        const tag = (m.dataset.tag || "").toLowerCase();
        const match = !q || name.includes(q) || tag.includes(q);
        m.classList.toggle("hidden", !match);
        if (match && m.dataset.tag !== "layout") shown++;
      });
      if (searchCount) {
        searchCount.textContent = q
          ? shown + " of " + moduleCount + " modules"
          : moduleCount + " modules";
        searchCount.setAttribute("aria-live", "polite");
      }
      if (modsEmpty) modsEmpty.classList.toggle("show", shown === 0);
    });
  }

  /* ---------- typewriter: custom module names ---------- */
  const typeEl = document.querySelector(".typewrap .type");
  if (typeEl) {
    const names = [
      "battery — % remaining, health, cycles",
      "weather — temp + condition for your city",
      "git — branch + dirty state in the cwd",
      "fsdeep — disk usage, deeper than df",
      "container — detected via /proc cgroup v2",
      "context — git, container, ssh, project",
    ];
    if (prefersReduced) {
      typeEl.textContent = names[0];
    } else {
      let i = 0;
      let char = 0;
      let deleting = false;
      const tick = () => {
        const word = names[i];
        typeEl.textContent = word.slice(0, char);
        if (!deleting) {
          char++;
          if (char > word.length) {
            deleting = true;
            setTimeout(tick, 1900);
            return;
          }
          setTimeout(tick, 26);
        } else {
          char -= 3;
          if (char <= 0) {
            char = 0;
            deleting = false;
            i = (i + 1) % names.length;
            setTimeout(tick, 350);
            return;
          }
          setTimeout(tick, 12);
        }
      };
      tick();
    }
  }

  /* ---------- live gauge mini-demo (card D): organic ticker ---------- */
  const gaugeRow = document.querySelector(".gauge-row");
  if (gaugeRow && !prefersReduced && "IntersectionObserver" in window) {
    const gauge = gaugeRow.querySelector(".gauge");
    const gaugeVal = gaugeRow.querySelector(".gauge span");
    const bars = gaugeRow.querySelectorAll(".spark i");
    let timer = null;
    let pct = 37;
    // seed heights from the current bars so the first tick is a continuation
    let heights = Array.from(bars, (b) =>
      parseFloat(b.style.getPropertyValue("--h")) || 0.4
    );

    const tick = () => {
      // don't waste ticks while the tab is hidden
      if (document.hidden) return;
      // organic random walk, clamped to a plausible CPU band
      pct = Math.max(18, Math.min(72, pct + (Math.random() * 14 - 7)));
      const rounded = Math.round(pct);
      if (gauge) gauge.style.setProperty("--p", rounded);
      if (gaugeVal) gaugeVal.textContent = rounded + "%";
      // scroll the sparkline: drop the oldest sample, append a new one
      heights = heights.slice(1);
      const last = heights[heights.length - 1] || 0.4;
      heights.push(
        Math.max(0.12, Math.min(0.95, last + (Math.random() * 0.36 - 0.18)))
      );
      bars.forEach((b, i) => b.style.setProperty("--h", heights[i].toFixed(2)));
    };

    const gaugeIo = new IntersectionObserver(
      (entries) => {
        entries.forEach((e) => {
          if (e.isIntersecting && !timer) {
            timer = setInterval(tick, 1400);
          } else if (!e.isIntersecting && timer) {
            clearInterval(timer);
            timer = null;
          }
        });
      },
      { threshold: 0.35 }
    );
    gaugeIo.observe(gaugeRow);
  }

  /* ---------- platform hint ---------- */
  const platHint = document.getElementById("plat-hint");
  if (platHint) {
    const ua = navigator.userAgent || "";
    let os = "linux";
    if (/mac/i.test(ua)) os = "macOS";
    const arch = /arm|aarch64/i.test(ua) ? "ARM64" : "x86_64";
    platHint.innerHTML =
      'detected: <span class="detected">' + os + " " + arch +
      "</span> — binary available";
  }

  /* ---------- footer year ---------- */
  const year = document.getElementById("year");
  if (year) year.textContent = new Date().getFullYear();

  /* ---------- back to top ---------- */
  const btt = document.getElementById("btt");
  if (btt) {
    const heroEl = document.getElementById("top");
    const showBtt = () => {
      const past = heroEl
        ? heroEl.getBoundingClientRect().bottom < 0
        : window.scrollY > 400;
      btt.classList.toggle("visible", past);
    };
    window.addEventListener("scroll", showBtt, { passive: true });
    btt.addEventListener("click", () => window.scrollTo({ top: 0, behavior: "smooth" }));
    showBtt();
  }
})();
