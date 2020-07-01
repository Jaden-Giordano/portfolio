module.exports = {
  chainWebpack: (config) => {
    config.module
          .rule('wasm')
          .test(/\.wasm(\?.*)?$/)
          .type('javascript/auto')
          .use('wasm-loader')
          .loader('wasm-loader');
    config.resolve.extensions.add('.wasm');
  },
  lintOnSave: false,
};
