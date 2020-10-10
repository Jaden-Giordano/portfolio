const WorkerPlugin = require("worker-plugin");
const CompressionPlugin = require("compression-webpack-plugin");

module.exports = {
  configureWebpack: {
    plugins: [
      new CompressionPlugin({
        deleteOriginalAssets: true,
        exclude: "index.html",
      }),
    ],
  },
  lintOnSave: false,
  chainWebpack: (config) => {
    config
      .plugin("worker")
      .use(WorkerPlugin, [{ filename: "[name].worker.js" }]);
    config.plugin("html").tap((args) => {
      args[0].title = "greymouth.io";
      return args;
    });
  },
};
