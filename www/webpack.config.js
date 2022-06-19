const CopyWebpackPlugin = require("copy-webpack-plugin");
const path = require('path');
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const HtmlWebpackPlugin = require("html-webpack-plugin");

module.exports = {
  entry: "./bootstrap.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "bootstrap.js",
  },
  mode: "development",
  resolve: {
    extensions: [".js", ".wasm"]
  },
  watchOptions: {
    aggregateTimeout: 200,
    poll: 1000,
  },
  plugins: [
    new HtmlWebpackPlugin({template: "index.html"}),
    // new WasmPackPlugin({
    //   crateDirectory: path.join(__dirname, '../'),
    //   outDir: path.join(__dirname, '../pkg'),
    //   args: "--log-level warn",
    //   extraArgs: '--features=wasm',
    // })
  ],
  experiments: {
    asyncWebAssembly: true
  }
};
