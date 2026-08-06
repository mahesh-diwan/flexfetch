/* flexfetch landing page — interactions */
(function () {
  "use strict";

  /* ---------- toast ---------- */
  const toastEl = document.getElementById("toast");
  let toastTimer = null;
  function toast(msg) {
    toastEl.textContent = msg;
    toastEl.classList.add("show");
    clearTimeout(toastTimer);
    toastTimer = setTimeout(() => toastEl.classList.remove("show"), 2200);
  }

  /* ---------- hero: load the real rendered output ---------- */
  const hero = document.getElementById("hero-body");
  if (hero) {
    const fallback =
      "mahesh-diwan@cachyos-x8664\n────────────────────────────\n── System ──\nOS       : CachyOS\nHost     : cachyos-x8664\nKernel   : Linux 7.1.3-2-cachyos x86_64\nUptime   : 4d 18h 47m\n── Software ──\nShell    : fish\nDE       : Hyprland\n── Hardware ──\nCPU      : 12th Gen Intel Core i5-12450H (12 cores)\nMemory   : 8.3 GiB / 15.3 GiB (54%)\nGPU      : i915\nDisk     : /: 476.6G / 398.4G 84%";
    fetch("assets/hero.html")
      .then((r) => (r.ok ? r.text() : Promise.reject(r.status)))
      .then((html) => {
        hero.innerHTML = html;
        const cursor = document.createElement("span");
        cursor.className = "cursor";
        hero.appendChild(cursor);
      })
      .catch(() => {
        hero.textContent = fallback;
      });
  }

  /* ---------- copy buttons ---------- */
  document.querySelectorAll("[data-copy]").forEach((btn) => {
    btn.addEventListener("click", () => {
      const id = btn.dataset.copy;
      const text = (document.getElementById(id) || {}).textContent || "";
      const done = () => {
        btn.classList.add("copied");
        btn.textContent = "copied ✓";
        setTimeout(() => {
          btn.classList.remove("copied");
          btn.textContent = "copy";
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
    try { document.execCommand("copy"); done(); } catch (e) { /* ignore */ }
    document.body.removeChild(ta);
  }

  /* ---------- theme swatches → copy command ---------- */
  document.querySelectorAll(".theme").forEach((card) => {
    card.addEventListener("click", () => {
      const cmd = card.querySelector(".tcmd");
      if (!cmd) return;
      const text = cmd.textContent.trim();
      if (navigator.clipboard && window.isSecureContext) {
        navigator.clipboard.writeText(text).then(() => toast(`copied: ${text}`));
      } else {
        toast(`theme: ${text}`);
      }
    });
  });

  /* ---------- module filter ---------- */
  const search = document.getElementById("mod-search");
  const chips = document.querySelectorAll("#mod-chips .chip");
  const mods = Array.from(document.querySelectorAll("#mod-grid .mod"));
  const empty = document.getElementById("mod-empty");
  let activeCat = "all";

  function applyFilter() {
    const q = (search.value || "").trim().toLowerCase();
    let shown = 0;
    mods.forEach((m) => {
      const name = m.textContent.trim().toLowerCase();
      const cat = m.dataset.cat;
      const matchCat = activeCat === "all" || cat === activeCat;
      const matchQ = !q || name.includes(q);
      const visible = matchCat && matchQ;
      m.classList.toggle("hidden", !visible);
      if (visible) shown++;
    });
    empty.classList.toggle("show", shown === 0);
  }
  search.addEventListener("input", applyFilter);
  chips.forEach((chip) => {
    chip.addEventListener("click", () => {
      chips.forEach((c) => c.classList.remove("active"));
      chip.classList.add("active");
      activeCat = chip.dataset.cat;
      applyFilter();
    });
  });

  /* ---------- reveal on scroll ---------- */
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

  /* ---------- nav state ---------- */
  const nav = document.getElementById("nav");
  const onScroll = () => nav.classList.toggle("scrolled", window.scrollY > 8);
  window.addEventListener("scroll", onScroll, { passive: true });
  onScroll();

  /* ---------- footer year ---------- */
  const year = document.getElementById("year");
  if (year) year.textContent = new Date().getFullYear();
})();
