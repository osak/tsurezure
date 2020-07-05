const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const CopyPlugin = require('copy-webpack-plugin');
const webpack = require('webpack');

let constants;
if (process.env.NODE_ENV == 'production') {
    constants = {
        URL_BASE: JSON.stringify('https://tsurezure.herokuapp.com'),
    };
} else {
    constants = {
        URL_BASE: JSON.stringify('http://localhost:8000'),
    }
}
 
module.exports = {
    entry: {
        index: './web/index.tsx',
        admin: './web/index_admin.tsx',
    },
    devtool: 'source-map',
    devServer: {
        host: '0.0.0.0',
        historyApiFallback: true,
    },

    module: {
        rules: [
            {
                test: /\.tsx?/,
                use: 'ts-loader',
                exclude: /node_modules/,
            },
        ],
    },
    resolve: {
        extensions: ['.tsx', '.ts', '.js'],
    },
    output: {
        filename: '[name].bundle.js',
        path: path.resolve(__dirname, 'web-dist'),
        publicPath: '/',
    },
    plugins: [
        new HtmlWebpackPlugin({
            chunks: ['index'],
            filename: 'index.html',
            template: './web/index.html',
            hash: true,
        }),
        new HtmlWebpackPlugin({
            chunks: ['admin'],
            filename: 'admin.html',
            template: './web/index.html',
            hash: true,
        }),

        new webpack.DefinePlugin(constants),
        new CopyPlugin({
            patterns: [
                { from: './web/style.css', to: 'style.css' }
            ]
        }),
        new webpack.IgnorePlugin(/^\.\/locale$/, /moment$/),
    ],
}