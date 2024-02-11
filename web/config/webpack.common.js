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
  plugins: [
    new CopyWebpackPlugin([
      {
        from: paths.src + "/assets",
        to: "assets",
        globOptions: {
          dot: true,
          ignore: ["*.DS_Store"],
        },
      }
    ]),
    new HtmlWebpackPlugin({
      title: "National Missing and Unidentified Persons",
      template: paths.public + "/index.html",
      filename: "index.html",
    }),
    new WasmPackPlugin({
      crateDirectory: path.join(__dirname, '../'),
      outDir: path.join(__dirname, '../pkg'),
      args: "--log-level warn",
      extraArgs: '--features=wasm',
    })
  ],
  module: {
    rules: [
      { test: /\.(js|jsx)$/, exclude: /node_modules/, use: ["babel-loader"] },
      {
        test: /\.(scss|css)$/,
        use: [
          "style-loader",
          {
            loader: "css-loader",
            options: { sourceMap: true, importLoaders: 1 },
          },
          { loader: "sass-loader", options: { sourceMap: true } },
        ],
      },
      // Images: Copy image files to build folder
      { test: /\.(?:ico|gif|png|jpg|jpeg)$/i, type: "asset/resource" },
      // Fonts and SVGs: Inline files
      { test: /\.(woff(2)?|eot|ttf|otf|svg|)$/, type: "asset/inline" },
    ]
  }
}
