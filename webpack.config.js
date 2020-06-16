const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');

module.exports = {
    entry: './web/index.tsx',
    devtool: 'source-map',
    devServer: {
        host: '0.0.0.0',
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
        filename: 'bundle.js',
        path: path.resolve(__dirname, 'web-dist'),
    },
    plugins: [
        new HtmlWebpackPlugin({
            template: './web/index.html'
        })
    ],
}