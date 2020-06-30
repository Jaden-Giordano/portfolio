module.exports = {
  configureWebpack: {
    module: {
      rules: [{
        test: /\.wasm$/,
        loader: 'wasm-loader',
      }],
    },
  },
  lintOnSave: false,
};
