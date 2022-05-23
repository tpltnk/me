const path = require('path');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');
const CopyWebpackPlugin = require('copy-webpack-plugin');

const distPath = path.resolve(__dirname, "dist");
const staticPath = path.resolve(__dirname, "static");

module.exports = (env, argv) => {
  return {
    devServer: {
      contentBase: distPath,
      host: '0.0.0.0',
      compress: argv.mode === 'production',
      port: 8000
    },
    experiments: {
      syncWebAssembly: true,
      asyncWebAssembly: true,
    },
    entry: './bootstrap.js',
    output: {
      path: distPath,
      filename: "portfolio.js",
      webassemblyModuleFilename: "portfolio.wasm"
    },
    module: {
      rules: [
        {
          test: /\.css$/i,
          include: staticPath,
          use: [
            'style-loader',
            'css-loader',
            'postcss-loader'
          ],
        },
      ],
    },
    plugins: [
      new CopyWebpackPlugin({
        patterns: [
          { from: './static', to: distPath },
        ],
      }),
      new WasmPackPlugin({
        crateDirectory: ".",
        extraArgs: "--no-typescript",
      })
    ],
    watch: argv.mode !== 'production'
  };
};
