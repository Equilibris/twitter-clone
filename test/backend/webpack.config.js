const path = require('path')
const fs = require('fs')
const { CleanWebpackPlugin } = require('clean-webpack-plugin')

const entry = {}

for (const file of fs.readdirSync(path.join(__dirname, './src'))) {
	console.log(file)
	if (/\.ts/g.test(file)) entry[file.replace(/\..+$/, '')] = `./src/${file}`
}

module.exports = {
	mode: 'production',
	entry,
	output: {
		path: path.join(__dirname, 'dist'),
		libraryTarget: 'commonjs',
		filename: '[name].js',
	},
	resolve: {
		extensions: ['.ts', '.js'],
	},
	module: {
		rules: [
			{
				test: /\.ts$/,
				use: 'babel-loader',
			},
		],
	},
	target: 'web',
	externals: /^k6(\/.*)?/,
	stats: {
		colors: true,
	},
	plugins: [new CleanWebpackPlugin()],
}
