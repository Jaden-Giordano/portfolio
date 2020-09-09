onmessage = (event) => {
  let { message } = event.data;
  if (message === 'LOAD') {
    import('@portfolio/webgl').then(module => {
      const client = new module.FolioClient(event.data.canvas);
      const render = () => {
        client.update();
        client.render();
        requestAnimationFrame(render);
      };
      requestAnimationFrame(render);
    });
  }
}
