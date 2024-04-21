const CopyWebpackPlugin = require('copy-webpack-plugin');
const MiniCssExtractPlugin = require("mini-css-extract-plugin");
const StylelintPlugin = require('stylelint-webpack-plugin');

module.exports = {
    entry: ['./src/styles/index.scss'],
    plugins: [
        new CopyWebpackPlugin({
            patterns: [
                {from: 'static', to: 'bundle'}
            ]
        }),
        new MiniCssExtractPlugin({
            filename: "bundle/[name].css",
        }),
        new StylelintPlugin({
            context: "./src/styles",
            fix: true,
            threads: true,
        }),
    ],
    module: {
        rules: [
            {
                test: /\.s[ac]ss$/i,
                use: [MiniCssExtractPlugin.loader, "css-loader", "postcss-loader", "sass-loader"],
            },
        ],
    },
    mode: "production"
};
