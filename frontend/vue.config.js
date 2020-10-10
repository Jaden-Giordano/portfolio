const WorkerPlugin = require("worker-plugin");
const ExtraWatchWebpackPlugin = require("extra-watch-webpack-plugin");
const WebpackShellPlugin = require("webpack-shell-plugin");

module.exports = {
  configureWebpack: {
    plugins: [
      new ExtraWatchWebpackPlugin({
        dirs: ["../webgl/src"],
      }),
      new WebpackShellPlugin({
        onBuildStart: ["yarn workspace @portfolio/webgl build"],
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
