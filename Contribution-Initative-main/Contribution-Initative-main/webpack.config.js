require("dotenv").config();
const path = require("path");
const webpack = require("webpack");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const TerserPlugin = require("terser-webpack-plugin");
const CopyPlugin = require("copy-webpack-plugin");

const isDevelopment = process.env.NODE_ENV !== "production";

const frontendDirectory = "contribution_initative1_frontend";

const frontend_entry = path.join("src", frontendDirectory, "src", "index.html");
const organization_entry = path.join("src", frontendDirectory, "src", "organization.html");
const login_entry = path.join("src", frontendDirectory, "src", "login.html");
const profile_entry = path.join("src", frontendDirectory, "src", "profile.html");
const register_entry = path.join("src", frontendDirectory, "src", "register.html");



module.exports = {
  target: "web",
  mode: isDevelopment ? "development" : "production",
  entry: {
    // The frontend.entrypoint points to the HTML file for this build, so we need
    // to replace the extension to `.js`.
    index: path.join(__dirname, frontend_entry).replace(/\.html$/, ".js"),
    organization: path.join(__dirname, organization_entry).replace(/\.html$/, ".js"),
    login: path.join(__dirname, login_entry).replace(/\.html$/, ".js"),
    profile: path.join(__dirname, profile_entry).replace(/\.html$/, ".js"),
    register: path.join(__dirname, register_entry).replace(/\.html$/, ".js"),

  },
  devtool: isDevelopment ? "source-map" : false,
  optimization: {
    minimize: !isDevelopment,
    minimizer: [new TerserPlugin()],
  },
  resolve: {
    extensions: [".js", ".ts", ".jsx", ".tsx"],
    fallback: {
      assert: require.resolve("assert/"),
      buffer: require.resolve("buffer/"),
      events: require.resolve("events/"),
      stream: require.resolve("stream-browserify/"),
      util: require.resolve("util/"),
    },
  },
  output: {
    filename: "[name].js",
    path: path.join(__dirname, "dist", frontendDirectory),
  },

  // Depending in the language or framework you are using for
  // front-end development, add module loaders to the default
  // webpack configuration. For example, if you are using React
  // modules and CSS as described in the "Adding a stylesheet"
  // tutorial, uncomment the following lines:
  // module: {
  //  rules: [
  //    { test: /\.(ts|tsx|jsx)$/, loader: "ts-loader" },
  //    { test: /\.css$/, use: ['style-loader','css-loader'] }
  //  ]
  // },
  plugins: [
    // new HtmlWebpackPlugin({
    //   template: path.join(__dirname, frontend_entry),
    //   cache: false,
    // }),
    new HtmlWebpackPlugin({
      template: path.join(__dirname, frontend_entry),
      filename: "index.html",
      chunks: ["index"],
      cache: false,
    }),
    new HtmlWebpackPlugin({
      template: path.join(__dirname, organization_entry),
      filename:'organization.html',
      chunks: ["organization"],
    }),
    new HtmlWebpackPlugin({
      template: path.join(__dirname, login_entry),
      filename:'login.html',
      chunks: ["login"],
    }),
    new HtmlWebpackPlugin({
      template: path.join(__dirname, profile_entry),
      filename:'profile.html',
      chunks: ["profile"],
    }),
    new HtmlWebpackPlugin({
      template: path.join(__dirname, register_entry),
      filename:'register.html',
      chunks: ["register"],
    }),
    new webpack.EnvironmentPlugin([
      ...Object.keys(process.env).filter((key) => {
        if (key.includes("CANISTER")) return true;
        if (key.includes("DFX")) return true;
        return false;
      }),
    ]),
    new webpack.ProvidePlugin({
      Buffer: [require.resolve("buffer/"), "Buffer"],
      process: require.resolve("process/browser"),
    }),
    new CopyPlugin({
      patterns: [
        {
          from: `src/${frontendDirectory}/src/.ic-assets.json*`,
          to: ".ic-assets.json5",
          noErrorOnMissing: true,
        },
      ],
    }),
  ],
  // proxy /api to port 4943 during development.
  // if you edit dfx.json to define a project-specific local network, change the port to match.
  devServer: {
    proxy: {
      "/api": {
        target: "http://127.0.0.1:4943",
        changeOrigin: true,
        pathRewrite: {
          "^/api": "/api",
        },
      },
    },
    static: path.resolve(__dirname, "src", frontendDirectory, "assets"),
    hot: true,
    watchFiles: [path.resolve(__dirname, "src", frontendDirectory)],
    liveReload: true,
  },
};
