import('./target/js/canvas')
  .then(canvasExample => {
    console.log('Loaded the canvasExample', canvasExample)
    function loop () {
      try {
        canvasExample.draw()
        requestAnimationFrame(loop);
      } catch (error) {
        console.error('Error drawing the canvas', error)
      }
    }
    loop();
  })
  .catch((error) => {
    console.error('There was an error when importing the file', error)
  });
