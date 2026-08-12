/* Collapses each card's stacked fact slides into a pager.
 *
 * The HTML ships every slide visible. This script is the ONLY thing that hides
 * any of them, and every hiding rule in the stylesheet is scoped under the
 * `data-paged` attribute set below. So a crawler, a printer, Reader mode, a
 * scripting-off screen reader, or a thrown exception in this file all leave the
 * full text on the page. The pager is an enhancement; the document is the truth.
 *
 * Slide 1 is written to stand alone (enforced by a test in content.rs), so a
 * visitor who never presses anything still gets the whole claim. */
(function () {
  var wrap = document.querySelector('.builds');
  if (!wrap) return;

  var cards = [].slice.call(document.querySelectorAll('.build'));
  if (!cards.length) return;

  cards.forEach(function (card) {
    var slides = [].slice.call(card.querySelectorAll('.slide'));
    var dots = [].slice.call(card.querySelectorAll('.dot'));
    var prev = card.querySelector('.pg-prev');
    var next = card.querySelector('.pg-next');
    if (slides.length < 2 || !prev || !next) return;

    var at = 0;

    function show(i) {
      at = Math.max(0, Math.min(slides.length - 1, i));
      slides.forEach(function (s, n) { s.classList.toggle('is-active', n === at); });
      dots.forEach(function (d, n) {
        if (n === at) { d.setAttribute('aria-current', 'true'); }
        else { d.removeAttribute('aria-current'); }
      });
      // Ends are disabled rather than wrapped: wrapping from the last slide back
      // to the claim reads as a glitch, and there is no progress cue to explain it.
      prev.disabled = at === 0;
      next.disabled = at === slides.length - 1;
    }

    prev.addEventListener('click', function () { show(at - 1); });
    next.addEventListener('click', function () { show(at + 1); });
    dots.forEach(function (d, n) {
      d.addEventListener('click', function () { show(n); });
    });

    // Arrow keys page only while focus is inside this card, so they never hijack
    // scrolling for someone reading the rest of the page.
    card.addEventListener('keydown', function (e) {
      if (e.key === 'ArrowLeft') { show(at - 1); e.preventDefault(); }
      else if (e.key === 'ArrowRight') { show(at + 1); e.preventDefault(); }
    });

    show(0);
  });

  wrap.setAttribute('data-paged', '');

  /* Autoplay is a preference, not a given. Under prefers-reduced-motion the
     poster frame stands in and the video gains its own controls, so the content
     is still reachable without anything moving on its own. */
  var still = window.matchMedia && matchMedia('(prefers-reduced-motion: reduce)').matches;
  if (still) {
    [].slice.call(document.querySelectorAll('video.shot')).forEach(function (v) {
      v.autoplay = false;
      v.controls = true;
      v.pause();
    });
  }

  /* ── enlarge ────────────────────────────────────────────────────────────
   * A native <dialog>, not a hand-rolled overlay: showModal() gives the
   * backdrop, the focus trap, Escape-to-close and inert-behind for free, and
   * gets them right in ways a div rebuild usually does not.
   *
   * The morph is the View Transitions API. Giving the card video and the
   * dialog video the same view-transition-name makes the browser tween between
   * the two boxes instead of popping. Where it is unsupported the dialog just
   * opens instantly -- same behaviour, no animation, nothing to feature-detect
   * beyond the one call. */
  var dlg = document.createElement('dialog');
  dlg.className = 'lightbox';
  var big = document.createElement('video');
  big.muted = true; big.loop = true; big.playsInline = true; big.controls = true;
  var close = document.createElement('button');
  close.type = 'button'; close.className = 'lb-close';
  close.setAttribute('aria-label', 'Close'); close.textContent = '×';
  dlg.appendChild(big); dlg.appendChild(close);
  document.body.appendChild(dlg);

  var NAME = 'zoomed';
  var origin = null;

  function swap(fn) {
    // startViewTransition takes a callback that mutates the DOM; without support
    // we just run it. Either way the same code path produces the same end state.
    if (document.startViewTransition && !still) document.startViewTransition(fn);
    else fn();
  }

  function open(btn) {
    var card = btn.closest('.build');
    origin = card ? card.querySelector('video.shot') : null;
    big.src = btn.getAttribute('data-zoom');
    big.poster = btn.getAttribute('data-poster');
    if (origin) origin.style.viewTransitionName = NAME;
    swap(function () {
      if (origin) origin.style.viewTransitionName = '';
      big.style.viewTransitionName = NAME;
      dlg.showModal();
      if (!still) big.play().catch(function () {});
    });
  }

  function shut() {
    swap(function () {
      big.style.viewTransitionName = '';
      if (origin) origin.style.viewTransitionName = NAME;
      dlg.close();
    });
    // hand the name back after the transition so the card video is paintable again
    setTimeout(function () { if (origin) origin.style.viewTransitionName = ''; }, 400);
    big.pause();
  }

  [].slice.call(document.querySelectorAll('.zoom')).forEach(function (btn) {
    btn.addEventListener('click', function () { open(btn); });
  });
  close.addEventListener('click', shut);
  // Escape already closes a modal dialog; this catches the click-outside case,
  // which <dialog> does not handle on its own.
  dlg.addEventListener('click', function (e) { if (e.target === dlg) shut(); });
  dlg.addEventListener('close', function () { big.pause(); });
})();
