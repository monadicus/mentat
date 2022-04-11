const webpack = require('webpack');
const path = require('path');
const { CleanWebpackPlugin } = require('clean-webpack-plugin');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const ReactRefreshWebpackPlugin = require('@pmmmwh/react-refresh-webpack-plugin');

const development = process.env.MODE !== 'production';

const SWC_LOADER = {
  loader: 'swc-loader',
  options: {
    jsc: {
      keepClassNames: true,
      loose: true,
      parser: {
        syntax: 'typescript',
        tsx: true,
        dynamicImport: true,
        exportDefaultFrom: true,
        experimental: { styledComponent: true },
      },
      transform: {
        react: {
          runtime: 'automatic',
          refresh: development,
        },
      },
      target: 'es2016',
    },
  },
};

module.exports = {
  mode: development ? 'development' : 'production',
  devtool: development && 'source-map',
  entry: './src/root.tsx',
  cache: { type: 'filesystem' },
  output: {
    path: path.resolve(__dirname, 'dist'),
    filename: `[name].${development ? 'dev' : '[fullhash:7]'}.js`,
    publicPath: '/',
  },
  resolve: {
    cacheWithContext: true,
    extensions: ['', '.js', '.jsx', '.ts', '.tsx'],
  },
  watchOptions: { poll: true, ignored: /node_modules/ },
  module: {
    rules: [
      {
        test: /\.ya?ml$/,
        type: 'json',
        use: [{ loader: 'yaml-loader', options: { asJSON: true } }],
      },
      {
        test: /\.js(on|x)?$/,
        include: path.resolve(__dirname, 'src'),
        exclude: /node_modules/,
        use: [SWC_LOADER],
      },
      {
        test: /\.tsx?$/,
        include: path.resolve(__dirname, 'src'),
        exclude: /node_modules/,
        use: [SWC_LOADER],
      },
      {
        test: /\.css$/,
        use: [
          'style-loader',
          {
            loader: 'css-loader',
            options: { import: true },
          },
        ],
      },
      {
        test: /\.(png|woff|woff2|eot|ttf|svg|wav|ico)$/,
        type: 'asset',
        parser: {
          dataUrlCondition: {
            maxSize: 16 * 1024,
          },
        },
      },
    ],
  },
  plugins: [
    new CleanWebpackPlugin(),
    new HtmlWebpackPlugin({
      // TODO: create a nice favicon
      // favicon: './res/favicon.ico',
      template: './src/index.html',
      publicPath: '/',
    }),
    development &&
      new webpack.EvalSourceMapDevToolPlugin({
        exclude: ['vendor'],
        columns: true,
        module: true,
      }),
    development &&
      new ReactRefreshWebpackPlugin({ overlay: { sockIntegration: 'whm' } }),
  ].filter(Boolean),
  devServer: {
    devMiddleware: { writeToDisk: true },
    static: [path.resolve(__dirname, 'dist')],
    hot: true,
    host: '0.0.0.0',
    port: 3000,
    headers: { 'Access-Control-Allow-Origin': '*' },
    server: { type: 'http' },
    // lets react router handle 404s rather than the web server
    historyApiFallback: { index: '/' },
  },
};
