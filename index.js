import('./target/js/canvas')
  .then(canvasExample => {
    console.log('window.canvasExample')
    window.canvasExample = canvasExample
  })
  .catch((error) => {
    console.error('There was an error when importing the file', error)
  });
