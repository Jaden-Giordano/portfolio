const WorkerPlugin = require("worker-plugin");
const CompressionPlugin = require("compression-webpack-plugin");

module.exports = {
  lintOnSave: false,
  chainWebpack: (config) => {
    config
      .plugin("worker")
      .use(WorkerPlugin, [{ filename: "[name].worker.js" }]);
    config.plugin("html").tap((args) => {
      args[0].title = "aidanmadeit.xyz";
      return args;
    });
  },
};
