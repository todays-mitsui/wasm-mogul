const path = require("path");
const CopyPlugin = require("copy-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

const dist = path.resolve(__dirname, "dist");

module.exports = {
  mode: "production",
  entry: {
    index: "./js/index.js"
  },
  output: {
    path: dist,
    filename: "[name].js",
  },
  // resolve: {
  //   extensions: [".ts", ".tsx", ".js"],
  //   extensionAlias: {
  //     ".js": [".js", ".ts"],
  //     ".cjs": [".cjs", ".cts"],
  //     ".mjs": [".mjs", ".mts"]
  //   }
  // },
  // module: {
  //   rules: [
  //     { test: /\.([cm]?ts|tsx)$/, loader: "ts-loader" }
  //   ]
  // },
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

    new WasmPackPlugin({
      crateDirectory: path.resolve(__dirname, "ski2"),
      extraArgs: "--target bundler --mode normal",
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
