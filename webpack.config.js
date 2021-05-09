const path = require("path");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

const dist = path.resolve(__dirname, "dist");

module.exports = {
  mode: "production",
  entry: {
    index: "./js/index.js"
  },
  output: {
    path: dist,
    filename: "[name].js"
  },
  devServer: {
    contentBase: dist,
    host: "0.0.0.0",
    port: 8081,
  },
  module: {
    rules: [
      {
        test: /\.(png|jpg|gif|woff|woff2|eot|ttf|svg)$/i,
        use: [
          {
            loader: "url-loader",
            options: {
              limit: 8192,
            },
          },
        ],
      }
    ]
  },
  plugins: [
    // new CopyPlugin([
    //   path.resolve(__dirname, "static")
    // ]),

    new HtmlWebpackPlugin({
      title: "test",
    }),

    new WasmPackPlugin({
      crateDirectory: __dirname,
      forceMode: "release",
    }),
  ],
  experiments: {
    asyncWebAssembly: true,
    // topLevelAwait: true,
  }
};
