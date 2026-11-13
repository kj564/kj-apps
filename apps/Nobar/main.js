// Nobar — OS.js iframe application
// Loads Nobar streaming film in an iframe window
;(function() {
  const OSjs = window.OSjs || {};
  
  if (typeof OSjs.make === 'function') {
    OSjs.make('Nobar', (core, proc, win) => {
      const w = core.make('osjs/window', {
        id: 'Nobar',
        title: 'Nobar — Streaming Film Indie',
        dimension: { w: 960, h: 680 },
        position: { x: 80, y: 80 },
        onWindowDestroy: () => proc.destroy()
      });
      
      w.render(($content) => {
        $content.innerHTML = '<iframe src="https://kj564.github.io/Nobar/" style="width:100%;height:100%;border:0;display:block" allow="fullscreen;clipboard-read;clipboard-write;autoplay;encrypted-media;picture-in-picture" sandbox="allow-scripts allow-same-origin allow-downloads allow-popups allow-forms allow-pointer-lock allow-modals allow-presentation"></iframe>';
      });
      
      return w;
    });
  }
})();
