/* flexfetch landing page — interactions */
(function () {
  "use strict";

  /* ---------- smooth scroll for anchor links ---------- */
  document.querySelectorAll('a[href^="#"]').forEach(a => {
    a.addEventListener('click', e => {
      const target = document.querySelector(a.getAttribute('href'));
      if (target) {
        e.preventDefault();
        target.scrollIntoView({ behavior: 'smooth', block: 'start' });
      }
    });
  });

  /* ---------- toast ---------- */
  const toastEl = document.getElementById("toast");
  let toastTimer = null;
  function toast(msg) {
    toastEl.textContent = msg;
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

  /* ---------- active nav state ---------- */
  const sections = document.querySelectorAll('section[id]');
  const navLinks = document.querySelectorAll('.nav-links a');
  if ('IntersectionObserver' in window) {
    const sectionIo = new IntersectionObserver(entries => {
      entries.forEach(entry => {
        if (entry.isIntersecting) {
          navLinks.forEach(l => l.classList.remove('active'));
          const active = document.querySelector(`.nav-links a[href="#${entry.target.id}"]`);
          if (active) active.classList.add('active');
        }
      });
    }, { threshold: 0.3, rootMargin: '-80px 0px -40% 0px' });
    sections.forEach(s => sectionIo.observe(s));
  }

  /* ---------- hamburger menu ---------- */
  const hamburger = document.getElementById("hamburger");
  const navMobile = document.getElementById("nav-mobile");
  if (hamburger && navMobile) {
    function closeMenu() {
      hamburger.classList.remove("open");
      navMobile.classList.remove("open");
      hamburger.setAttribute("aria-expanded", "false");
    }
    function openMenu() {
      hamburger.classList.add("open");
      navMobile.classList.add("open");
      hamburger.setAttribute("aria-expanded", "true");
    }
    hamburger.addEventListener("click", () => {
      const isOpen = hamburger.classList.contains("open");
      isOpen ? closeMenu() : openMenu();
    });
    navMobile.querySelectorAll("a").forEach((a) => {
      a.addEventListener("click", closeMenu);
    });
    document.addEventListener("keydown", (e) => {
      if (e.key === "Escape" && hamburger.classList.contains("open")) {
        closeMenu();
        hamburger.focus();
      }
    });
  }

  /* ---------- footer year ---------- */
  const year = document.getElementById("year");
  if (year) year.textContent = new Date().getFullYear();

  /* ---------- hero terminal ---------- */
  const heroTerm = document.getElementById("hero-terminal");
  if (heroTerm) {
    fetch("assets/hero.html")
      .then(r => r.ok ? r.text() : "")
      .then(html => { if (html) heroTerm.innerHTML = html; })
      .catch(() => {});
  }

  /* ---------- back-to-top ---------- */
  const btt = document.getElementById("btt");
  if (btt) {
    const heroEl = document.getElementById("top");
    const showBtt = () => {
      const past = heroEl ? heroEl.getBoundingClientRect().bottom < 0 : window.scrollY > 400;
      btt.classList.toggle("visible", past);
    };
    window.addEventListener("scroll", showBtt, { passive: true });
    btt.addEventListener("click", () => window.scrollTo({ top: 0, behavior: "smooth" }));
    showBtt();
  }

  /* ---------- module search ---------- */
  const searchInput = document.getElementById("mod-search");
  const searchCount = document.getElementById("mod-search-count");
  const modGrid = document.getElementById("mod-grid");
  if (searchInput && modGrid) {
    const mods = modGrid.querySelectorAll(".mod");
    const total = mods.length;
    searchInput.addEventListener("input", () => {
      const q = searchInput.value.toLowerCase().trim();
      let shown = 0;
      mods.forEach(m => {
        const name = m.querySelector(".mname").textContent.toLowerCase();
        const match = !q || name.includes(q);
        m.style.display = match ? "" : "none";
        if (match) shown++;
      });
      searchCount.textContent = q ? `${shown} of ${total}` : "";
    });
  }

  /* ---------- platform hint ---------- */
  const platHint = document.getElementById("plat-hint");
  if (platHint) {
    const ua = navigator.userAgent || "";
    let os = "linux";
    if (/mac/i.test(ua)) os = "macOS";
    const arch = /arm|aarch64/i.test(ua) ? "ARM64" : "x86_64";
    platHint.innerHTML = `detected: <span class="detected">${os} ${arch}</span> — binary available`;
  }
})();
