onmessage = (event) => {
  import("@portfolio/webgl").then((module) => {
    const gl = event.data.canvas.getContext("webgl");
    const client = new module.FolioClient(gl);

    const render = () => {
      client.update();
      client.render();
      setTimeout(function () {
        requestAnimationFrame(render);
      }, 1000);
    };
    requestAnimationFrame(render);
  });
};
