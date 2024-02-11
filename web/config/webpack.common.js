const paths = require("./paths");
const path = require("path");

const CopyWebpackPlugin = require("copy-webpack-plugin");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

module.exports = {
  entry: [paths.src + "/bootstrap.js"],
  output: {
    path: paths.build,
    filename: "bootstrap.js",
  },
  experiments: {
    asyncWebAssembly: true
  },
  mode: "development",
  resolve: {
    fallback: {
      "fs": false,
    }
  },
  plugins: [
    new HtmlWebpackPlugin({
      template: paths.public + "/index.html",
      filename: "index.html",
    }),
    new WasmPackPlugin({
      crateDirectory: path.join(__dirname, '../../'),
      outDir: path.join(__dirname, '../../pkg'),
      args: "--log-level warn",
      extraArgs: '--features=wasm',
    })
  ],
  module: {
    rules: [
      { test: /\.(js|jsx)$/, exclude: /node_modules/, use: ["babel-loader"] },
      {
        test: /\.css$/i,
        use: ["style-loader", "css-loader", "postcss-loader"],
      },
    ]
  }
}
