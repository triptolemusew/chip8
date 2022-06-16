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
  plugins: [
    // new CopyWebpackPlugin(['index.html']),
    new HtmlWebpackPlugin(),
    new WasmPackPlugin({
      crateDirectory: path.join(__dirname, '../.'),
      extraArgs: '--features=wasm',
      outDir: "pkg"
    })
  ],
  experiments: {
    asyncWebAssembly: true
  }
};
