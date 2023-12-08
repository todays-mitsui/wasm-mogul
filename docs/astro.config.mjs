import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';

// https://astro.build/config
export default defineConfig({
	base: '/docs/',
	outDir: '../static/docs',
	integrations: [
		starlight({
			title: 'Tuber',
			social: {
				github: 'https://github.com/todays-mitsui/wasm-mogul',
			},
			customCss: [
				'./src/assets/custom.css',
			],
			sidebar: [
				{
					label: 'Guides',
					items: [
						// Each item here is one entry in the navigation menu.
						{ label: 'Example Guide', link: '/guides/example/' },
					],
				},
				{
					label: 'Reference',
					autogenerate: { directory: 'reference' },
				},
			],
		}),
	],
});
