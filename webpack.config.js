const path = require("path");
const CopyPlugin = require("copy-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

const dist = path.resolve(__dirname, "dist");

module.exports = {
  mode: "production",
  entry: {
    index: "./js/index.js",
    sw: "./js/sw.js",
  },
  output: {
    path: dist,
    filename: "[name].js",
  },
  devServer: {
    static: {
      directory: dist,
    },
  },
  plugins: [
    new CopyPlugin({
      patterns: [
        { from: path.resolve(__dirname, "static"), to: dist },
      ],
    }),

    new WasmPackPlugin({
      crateDirectory: path.resolve(__dirname, "ski"),

      watchDirectories: [
        path.resolve(__dirname, "tuber/src")
      ],
    }),
  ],
  experiments: {
    asyncWebAssembly: true,
  },
  devtool: 'source-map',
  performance: {
    assetFilter: function (assetFilename) {
      return !/\.(map|wasm)$/.test(assetFilename);
    },
  },
};
