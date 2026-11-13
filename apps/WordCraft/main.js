// WordCraft — OS.js iframe application
// Loads WordCraft WASM word processor in an iframe window
;(function() {
  const OSjs = window.OSjs || {};
  
  // Register as OS.js application provider
  if (typeof OSjs.make === 'function') {
    // OS.js v3 pattern: use make() to register app factory
    OSjs.make('WordCraft', (core, proc, win) => {
      // Create window with iframe
      const w = core.make('osjs/window', {
        id: 'WordCraft',
        title: 'WordCraft',
        dimension: { w: 960, h: 680 },
        position: { x: 50, y: 50 },
        onWindowDestroy: () => proc.destroy()
      });
      
      w.render(($content) => {
        $content.innerHTML = '<iframe src="' + OSjs.url('wordcraft/') + '" style="width:100%;height:100%;border:0;display:block" allow="fullscreen;clipboard-read;clipboard-write" sandbox="allow-scripts allow-same-origin allow-downloads allow-popups allow-forms allow-pointer-lock"></iframe>';
      });
      
      return w;
    });
  }
})();
