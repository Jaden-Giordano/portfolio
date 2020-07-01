module.exports = {
  chainWebpack: (config) => {
    config.module
          .rule('wasm')
          .test(/\.wasm(\?.*)?$/)
          .type('javascript/auto')
          .use('wasm-loader')
          .loader('wasm-loader');
  },
  configureWebpack: {
    resolve: {
      extensions: ['.wasm', '.vue', '.js'],
    },
  },
  lintOnSave: false,
};
