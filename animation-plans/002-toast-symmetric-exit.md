# 002 — Toast symmetric exit

- **Status**: TODO
- **Commit**: 804d424
- **Severity**: MEDIUM
- **Category**: Preventing a jarring change
- **Estimated scope**: 2 files (site/src/styles/global.css, site/src/layouts/Layout.astro), small

## Problem

The toast (`site/src/styles/global.css:1024-1045`) enters with a 600ms spring and 600ms opacity fade, but exits by _removing_ the `.show` class — which snaps it back to `transform: translateX(-50%) translateY(100px); opacity: 0` instantly. The spring entrance gets a hard teleport exit.

Current CSS (`site/src/styles/global.css:1024-1045`):

```css
.toast {
  position: fixed;
  bottom: 24px;
  left: 50%;
  transform: translateX(-50%) translateY(100px);
  background: var(--bg-2);
  border: 1px solid var(--border);
  border-radius: 6px;
  padding: 12px 20px;
  font: 13px var(--font-mono);
  color: var(--text-strong);
  z-index: 100;
  opacity: 0;
  transition:
    transform var(--duration-normal) var(--ease-spring),
    opacity var(--duration-normal) var(--ease-out);
}

.toast.show {
  transform: translateX(-50%) translateY(0);
  opacity: 1;
}
```

Current JS (`site/src/layouts/Layout.astro:104-111`):

```js
function toast(msg) {
  toastEl.textContent = msg;
  toastEl.classList.add("show");
  clearTimeout(toastTimer);
  toastTimer = setTimeout(() => toastEl.classList.remove("show"), 2200);
}
```

## Target

Give the toast a symmetric exit: a `.hiding` class that animates back out over 300ms, then removes the class. The enter path stays identical (600ms spring). Exit uses `--ease-out` (entering or exiting → ease-out per AUDIT.md §2) and stays under 300ms budget for a small popover.

```css
/* target — add .toast.hiding after .toast.show */
.toast.show {
  transform: translateX(-50%) translateY(0);
  opacity: 1;
}

.toast.hiding {
  transform: translateX(-50%) translateY(100px);
  opacity: 0;
  transition:
    transform 300ms var(--ease-out),
    opacity 200ms var(--ease-out);
}
```

```js
// target — Layout.astro toast()
function toast(msg) {
  toastEl.textContent = msg;
  toastEl.classList.remove("hiding");
  toastEl.classList.add("show");
  clearTimeout(toastTimer);
  toastTimer = setTimeout(() => {
    toastEl.classList.add("hiding");
    setTimeout(() => toastEl.classList.remove("hiding"), 300);
  }, 2200);
}
```

## Repo conventions to follow

- Easing tokens: `--ease-out: cubic-bezier(0.16, 1, 0.3, 1)` in `site/src/styles/global.css:31`.
- Exit pattern: entering or exiting → `ease-out` (AUDIT.md §2). The entrance already uses `--ease-spring` for its bounce; the exit should be a clean ease-out drop.
- The `.hiding` + delayed class removal pattern is the site's existing approach (no animation library; class-based toggling).

## Steps

1. In `site/src/styles/global.css`, after the `.toast.show` rule (line 1042-1045), add the `.toast.hiding` rule from Target.
2. In `site/src/layouts/Layout.astro`, replace the `toast()` function body (lines 104-111) with the target JS.

## Boundaries

- Do NOT change `.toast` base transition (the spring entrance stays).
- Do NOT change the 2200ms display duration.
- Do NOT touch `#btt`, `.btt`, or other toast-adjacent elements.
- Motion properties and JS class toggling only.

## Verification

- **Mechanical**: `cd site && npm run build` — builds clean.
- **Feel check**: click a copy button (triggers toast), watch it leave:
  - Toast exits by sliding back down to `translateY(100px)` and fading over 300ms — no teleport.
  - Spamming copy resets the timer; the `.hiding` class is removed before re-show, so a re-shown toast doesn't start from hidden opacity.
  - In DevTools Animations panel: exit is ~300ms, entrance ~600ms — distinct, not identical.
- **Done when**: toast visibly slides out before disappearing, every time, including rapid re-triggers.

**Reduced motion**: the toast already animates `transform` + `opacity` (GPU-friendly). At 300ms exit it's well within the popover budget. The site has no global reduced-motion override; a future one should keep the opacity fade while dropping the `translateY` movement.
