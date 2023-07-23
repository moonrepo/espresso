module.exports = {
	arrowParens: 'always',
	bracketSameLine: false,
	bracketSpacing: true,
	embeddedLanguageFormatting: 'auto',
	endOfLine: 'lf',
	printWidth: 100,
	proseWrap: 'always',
	semi: true,
	singleAttributePerLine: false,
	singleQuote: true,
	tabWidth: 2,
	trailingComma: 'all',
	useTabs: true,
	overrides: [
		{
			files: ['*.json', '*.yml', '*.yaml', '*.md', '*.mdx'],
			options: {
				useTabs: false,
			},
		},
		{
			files: ['*.json'],
			options: {
				parser: 'json-stringify',
			},
		},
	],
};
