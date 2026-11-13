// VuCall — OS.js iframe application
// Loads VuCall video call in an iframe window
;(function() {
  const OSjs = window.OSjs || {};
  
  if (typeof OSjs.make === 'function') {
    OSjs.make('VuCall', (core, proc, win) => {
      const w = core.make('osjs/window', {
        id: 'VuCall',
        title: 'VuCall — Terhubung lewat video call',
        dimension: { w: 960, h: 680 },
        position: { x: 110, y: 110 },
        onWindowDestroy: () => proc.destroy()
      });
      
      w.render(($content) => {
        $content.innerHTML = '<iframe src="https://kj564.github.io/VuCall/" style="width:100%;height:100%;border:0;display:block" allow="fullscreen;clipboard-read;clipboard-write;autoplay;encrypted-media;picture-in-picture;camera;microphone" sandbox="allow-scripts allow-same-origin allow-downloads allow-popups allow-forms allow-pointer-lock allow-modals allow-presentation"></iframe>';
      });
      
      return w;
    });
  }
})();
