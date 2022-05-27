const webpack = require('webpack');
const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const ReactRefreshWebpackPlugin = require('@pmmmwh/react-refresh-webpack-plugin');
const nodeExternals = require('webpack-node-externals');

const development = process.env.MODE !== 'production';

const SWC_LOADER_REACT = [
  {
    loader: 'babel-loader',
  },
  {
    loader: 'swc-loader',
    options: {
      parseMap: true,
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
  },
];

const common = {
  mode: development ? 'development' : 'production',
  devtool: development && 'source-map',
  cache: { type: 'filesystem' },
  resolve: {
    cacheWithContext: true,
    extensions: ['', '.js', '.jsx', '.ts', '.tsx'],
  },
  watchOptions: { poll: true, ignored: /node_modules/ },
};

module.exports = [
  {
    ...common,
    entry: './src/root.tsx',
    output: {
      path: path.resolve(__dirname, 'dist'),
      filename: `[name].${development ? 'dev' : '[fullhash:7]'}.js`,
      publicPath: '/',
    },
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
          use: SWC_LOADER_REACT,
        },
        {
          test: /\.tsx?$/,
          include: path.resolve(__dirname, 'src'),
          exclude: /node_modules/,
          use: SWC_LOADER_REACT,
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
      development && new webpack.HotModuleReplacementPlugin(),
      development && new ReactRefreshWebpackPlugin(),
    ].filter(Boolean),
    devServer: {
      devMiddleware: { writeToDisk: true },
      static: [path.resolve(__dirname, 'dist')],
      hot: true,
      host: '0.0.0.0',
      port: 2999,
      headers: { 'Access-Control-Allow-Origin': '*' },
      server: { type: 'http' },
      // lets react router handle 404s rather than the web server
      historyApiFallback: { index: '/' },
    },
  },
  {
    ...common,
    entry: './backend/main.ts',
    target: 'node',
    externals: [nodeExternals()],
    output: {
      path: path.resolve(__dirname, 'dist'),
      filename: 'server.js',
    },
    resolve: {
      extensions: ['.ts', '.js', '.json'],
    },
    module: {
      rules: [
        {
          test: /\.[jt]s$/,
          exclude: /(node_modules)/,
          use: {
            loader: 'swc-loader',
            options: {
              isModule: true,
              jsc: {
                target: 'es2020',
                parser: {
                  syntax: 'typescript',
                },
              },
            },
          },
        },
      ],
    },
  },
];
