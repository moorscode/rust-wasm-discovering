const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const RemovePlugin = require("remove-files-webpack-plugin");

module.exports = {
    entry: './index.js',
    output: {
        path: path.resolve(__dirname, 'dist'),
        filename: 'index.js',
    },
    plugins: [
        new HtmlWebpackPlugin({
            template: 'index.html'
        }),
        new WasmPackPlugin({
            crateDirectory: path.resolve(__dirname, ".")
        }),
        new RemovePlugin({
            before: {
                include: [
                    './dist',
                ],
            }
        }),
    ],
    mode: 'development',
    experiments: {
        asyncWebAssembly: true
    }
};