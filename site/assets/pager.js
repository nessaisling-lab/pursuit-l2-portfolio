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
})();
