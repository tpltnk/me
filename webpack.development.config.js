const { merge } = require("webpack-merge");
const common = require("./webpack.common.config.js");

module.exports = merge(common, {
  mode: 'development',
  devServer: {
    disablehostcheck: true,
    compress: false,
  },
  watch: true,
});
