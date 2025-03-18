// This script fixes the canvas size to make it stretch 100% to the sides
document.addEventListener('DOMContentLoaded', function () {
  // Function to set canvas to full width/height
  function fixCanvasSize() {
    const canvas = document.querySelector('canvas');
    if (canvas) {
      // Remove any inline styles that might be setting fixed dimensions
      canvas.style.width = '100%';
      canvas.style.height = '100%';
      canvas.style.maxWidth = '100vw';
      canvas.style.maxHeight = '100vh';

      // Remove any fixed width/height attributes
      // We'll keep the aspect ratio by using CSS
      canvas.removeAttribute('width');
      canvas.removeAttribute('height');
    }
  }

  // Run immediately
  fixCanvasSize();

  // Also run after a short delay to ensure it catches the canvas
  // even if it's created after this script runs
  setTimeout(fixCanvasSize, 100);
  setTimeout(fixCanvasSize, 500);

  // Create a MutationObserver to watch for changes to the DOM
  const observer = new MutationObserver(function (mutations) {
    mutations.forEach(function (mutation) {
      if (mutation.addedNodes.length) {
        fixCanvasSize();
      }
    });
  });

  // Start observing the document body for DOM changes
  observer.observe(document.body, { childList: true, subtree: true });
});

