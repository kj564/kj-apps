// Service Worker: inject COOP/COEP headers untuk SharedArrayBuffer
// GitHub Pages tidak support custom headers, jadi SW intercept + modify

const COOP_COEP_HEADERS = {
  'Cross-Origin-Opener-Policy': 'same-origin',
  'Cross-Origin-Embedder-Policy': 'require-corp',
};

self.addEventListener('install', e => self.skipWaiting());
self.addEventListener('activate', e => e.waitUntil(self.clients.claim()));

self.addEventListener('fetch', event => {
  const url = new URL(event.request.url);
  
  // Only intercept requests to our firefox folder
  if (!url.pathname.includes('/firefox/')) {
    return;
  }

  event.respondWith(
    fetch(event.request)
      .then(response => {
        // Clone the response to modify headers
        const newHeaders = new Headers(response.headers);
        for (const [key, value] of Object.entries(COOP_COEP_HEADERS)) {
          newHeaders.set(key, value);
        }
        
        return new Response(response.body, {
          status: response.status,
          statusText: response.statusText,
          headers: newHeaders
        });
      })
      .catch(err => {
        console.error('[SW] fetch error:', err);
        return new Response('SW error: ' + err.message, { status: 500 });
      })
  );
});
