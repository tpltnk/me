const merge = require("webpack-merge");
const common = require("./webpack.common.config.js");

module.exports = merge(common, {
  mode: 'production',
  devServer: {
    disableHostCheck: false,
    compress: true,
  },
  watch: false,
});
