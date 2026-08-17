/* ==========================================================================
   flexfetch — GSAP ScrollTrigger scroll choreography
   Depends on: gsap + ScrollTrigger (loaded via CDN in index.html)
   Respects prefers-reduced-motion — all animations disabled when reduced.
   ========================================================================== */
(function () {
  "use strict";

  // wait for GSAP to be available
  if (typeof gsap === "undefined" || typeof ScrollTrigger === "undefined") return;

  const prefersReduced = window.matchMedia("(prefers-reduced-motion: reduce)").matches;
  if (prefersReduced) return;

  gsap.registerPlugin(ScrollTrigger);

  /* ---------- hero: parallax fade-out on scroll ---------- */
  const heroText = document.querySelector(".hero-text");
  const heroTerm = document.querySelector(".term-wrap");
  if (heroText) {
    gsap.to(heroText, {
      y: -40,
      opacity: 0.3,
      ease: "none",
      scrollTrigger: {
        trigger: ".hero",
        start: "top top",
        end: "bottom top",
        scrub: 0.5,
      },
    });
  }
  if (heroTerm) {
    gsap.to(heroTerm, {
      y: -20,
      scale: 0.97,
      ease: "none",
      scrollTrigger: {
        trigger: ".hero",
        start: "top top",
        end: "bottom top",
        scrub: 0.5,
      },
    });
  }

  /* ---------- stats: staggered number reveal ---------- */
  const statNums = document.querySelectorAll(".stat-num");
  if (statNums.length) {
    gsap.from(statNums, {
      y: 30,
      opacity: 0,
      duration: 0.6,
      stagger: 0.12,
      ease: "power2.out",
      scrollTrigger: {
        trigger: ".stats",
        start: "top 80%",
        toggleActions: "play none none none",
      },
    });
  }

  /* ---------- feature cards: staggered entrance ---------- */
  const cards = document.querySelectorAll(".bento .card");
  if (cards.length) {
    gsap.from(cards, {
      y: 50,
      opacity: 0,
      duration: 0.7,
      stagger: 0.1,
      ease: "power3.out",
      scrollTrigger: {
        trigger: ".bento",
        start: "top 75%",
        toggleActions: "play none none none",
      },
    });
  }

  /* ---------- modules grid: staggered entrance ---------- */
  const modsGrid = document.querySelector(".mods-grid");
  if (modsGrid) {
    gsap.from(modsGrid, {
      y: 40,
      opacity: 0,
      duration: 0.8,
      ease: "power2.out",
      scrollTrigger: {
        trigger: ".mods-grid",
        start: "top 80%",
        toggleActions: "play none none none",
      },
    });
  }

  /* ---------- install cards: staggered slide-in ---------- */
  const instCards = document.querySelectorAll(".inst-card");
  if (instCards.length) {
    gsap.from(instCards, {
      x: 40,
      opacity: 0,
      duration: 0.6,
      stagger: 0.15,
      ease: "power2.out",
      scrollTrigger: {
        trigger: ".inst-stack",
        start: "top 75%",
        toggleActions: "play none none none",
      },
    });
  }

  /* ---------- section headings: slide up ---------- */
  const sectionHeads = document.querySelectorAll(".section-head, .mods-head, .inst-copy");
  if (sectionHeads.length) {
    gsap.from(sectionHeads, {
      y: 30,
      opacity: 0,
      duration: 0.7,
      stagger: 0.08,
      ease: "power2.out",
      scrollTrigger: {
        trigger: ".section",
        start: "top 80%",
        toggleActions: "play none none none",
      },
    });
  }

  /* ---------- marquee: accelerate on scroll ---------- */
  const marqueeTrack = document.querySelector(".marquee-track");
  if (marqueeTrack) {
    ScrollTrigger.create({
      trigger: ".marquee",
      start: "top bottom",
      end: "bottom top",
      onUpdate: (self) => {
        const speed = 46 - self.progress * 20; // 46s → 26s as user scrolls through
        marqueeTrack.style.animationDuration = speed + "s";
      },
    });
  }

  /* ---------- CTA: scale up heading ---------- */
  const ctaH2 = document.querySelector(".cta h2");
  if (ctaH2) {
    gsap.from(ctaH2, {
      scale: 0.92,
      opacity: 0,
      duration: 0.8,
      ease: "power2.out",
      scrollTrigger: {
        trigger: ".cta",
        start: "top 75%",
        toggleActions: "play none none none",
      },
    });
  }

  /* ---------- footer: fade in ---------- */
  const footer = document.querySelector("footer");
  if (footer) {
    gsap.from(footer, {
      opacity: 0,
      duration: 0.6,
      ease: "power2.out",
      scrollTrigger: {
        trigger: footer,
        start: "top 90%",
        toggleActions: "play none none none",
      },
    });
  }
})();
