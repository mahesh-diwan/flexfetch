/* ==========================================================================
   flexfetch — GSAP ScrollTrigger scroll choreography
   Depends on: gsap + ScrollTrigger (loaded via CDN in index.html)
   Respects prefers-reduced-motion.
   
   KEY: Uses fromTo + immediateRender:false so elements stay VISIBLE
   (their natural CSS state) until ScrollTrigger fires. After animation,
   clearProps removes inline styles so CSS takes back control.
   ========================================================================== */
(function () {
  "use strict";

  if (typeof gsap === "undefined" || typeof ScrollTrigger === "undefined") return;

  const prefersReduced = window.matchMedia("(prefers-reduced-motion: reduce)").matches;
  if (prefersReduced) return;

  gsap.registerPlugin(ScrollTrigger);

  /* helper: fromTo with immediateRender:false, clear inline after */
  function scrollReveal(targets, fromVars, scrollConfig) {
    gsap.fromTo(targets, fromVars, {
      ...fromVars,  // same props as "to" target (natural CSS state)
      clearProps: "all",
      ease: "power2.out",
      scrollTrigger: scrollConfig,
    });
  }

  /* ---------- hero: parallax fade-out on scroll (scrub — always active) ---------- */
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

  /* ---------- stats: staggered entrance ---------- */
  const statEls = document.querySelectorAll(".stat");
  if (statEls.length) {
    gsap.fromTo(statEls,
      { y: 30, opacity: 0 },
      {
        y: 0,
        opacity: 1,
        duration: 0.6,
        stagger: 0.12,
        ease: "power2.out",
        immediateRender: false,
        clearProps: "all",
        scrollTrigger: {
          trigger: ".stats",
          start: "top 85%",
          toggleActions: "play none none none",
        },
      }
    );
  }

  /* ---------- feature cards: staggered entrance ---------- */
  const cards = document.querySelectorAll(".bento .card");
  if (cards.length) {
    gsap.fromTo(cards,
      { y: 50, opacity: 0 },
      {
        y: 0,
        opacity: 1,
        duration: 0.7,
        stagger: 0.1,
        ease: "power3.out",
        immediateRender: false,
        clearProps: "all",
        scrollTrigger: {
          trigger: ".bento",
          start: "top 80%",
          toggleActions: "play none none none",
        },
      }
    );
  }

  /* ---------- modules grid: entrance ---------- */
  const modsGrid = document.querySelector(".mods-grid");
  if (modsGrid) {
    gsap.fromTo(modsGrid,
      { y: 40, opacity: 0 },
      {
        y: 0,
        opacity: 1,
        duration: 0.8,
        ease: "power2.out",
        immediateRender: false,
        clearProps: "all",
        scrollTrigger: {
          trigger: ".mods-grid",
          start: "top 85%",
          toggleActions: "play none none none",
        },
      }
    );
  }

  /* ---------- install cards: staggered slide-in ---------- */
  const instCards = document.querySelectorAll(".inst-card");
  if (instCards.length) {
    gsap.fromTo(instCards,
      { x: 40, opacity: 0 },
      {
        x: 0,
        opacity: 1,
        duration: 0.6,
        stagger: 0.15,
        ease: "power2.out",
        immediateRender: false,
        clearProps: "all",
        scrollTrigger: {
          trigger: ".inst-stack",
          start: "top 80%",
          toggleActions: "play none none none",
        },
      }
    );
  }

  /* ---------- section headings: slide up ---------- */
  const sectionHeads = document.querySelectorAll(".section-head, .mods-head, .inst-copy");
  if (sectionHeads.length) {
    gsap.fromTo(sectionHeads,
      { y: 24, opacity: 0 },
      {
        y: 0,
        opacity: 1,
        duration: 0.6,
        stagger: 0.08,
        ease: "power2.out",
        immediateRender: false,
        clearProps: "all",
        scrollTrigger: {
          trigger: ".section",
          start: "top 85%",
          toggleActions: "play none none none",
        },
      }
    );
  }

  /* ---------- marquee: speed linked to scroll ---------- */
  const marqueeTrack = document.querySelector(".marquee-track");
  if (marqueeTrack) {
    ScrollTrigger.create({
      trigger: ".marquee",
      start: "top bottom",
      end: "bottom top",
      onUpdate: (self) => {
        const speed = 46 - self.progress * 20;
        marqueeTrack.style.animationDuration = speed + "s";
      },
    });
  }

  /* ---------- CTA: scale up heading ---------- */
  const ctaH2 = document.querySelector(".cta h2");
  if (ctaH2) {
    gsap.fromTo(ctaH2,
      { scale: 0.94, opacity: 0 },
      {
        scale: 1,
        opacity: 1,
        duration: 0.7,
        ease: "power2.out",
        immediateRender: false,
        clearProps: "all",
        scrollTrigger: {
          trigger: ".cta",
          start: "top 80%",
          toggleActions: "play none none none",
        },
      }
    );
  }

  /* ---------- footer: fade in ---------- */
  const footer = document.querySelector("footer");
  if (footer) {
    gsap.fromTo(footer,
      { opacity: 0 },
      {
        opacity: 1,
        duration: 0.5,
        ease: "power2.out",
        immediateRender: false,
        clearProps: "all",
        scrollTrigger: {
          trigger: footer,
          start: "top 92%",
          toggleActions: "play none none none",
        },
      }
    );
  }
})();
