import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';
import remarkBreaks from 'remark-breaks';

// https://astro.build/config
export default defineConfig({
  base: '/doc/',
  outDir: '../dist/doc',
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
          label: '記法: Syntax',
          items: [
            { label: 'はじめに', link: '/syntax/about/' },
            { label: '識別子: Identifier', link: '/syntax/identifier/' },
            {
              label: '式: Expression',
              items: [
                { label: '変数: Variable', link: '/syntax/expression/#変数-variable' },
                { label: 'シンボル: Symbol', link: '/syntax/expression/#シンボル-symbol' },
                { label: '関数適用: Application', link: '/syntax/expression/#関数適用-application' },
                { label: 'ラムダ抽象: Lambda', link: '/syntax/expression/#ラムダ抽象-lambda' },
              ]
            },
            { label: '名前付き関数: Function', link: '/syntax/function/' },
          ],
        },
        {
          label: 'コマンド: Command',
          items: [
            { label: '簡約: Reduce', link: '/command/reduce/' },
            { label: '関数定義: Define', link: '/command/define/' },
            {
              label: 'コンテキスト: Context',
              items: [
                { label: '一覧: List', link: '/command/context/list/' },
                { label: '問い合わせ: Query', link: '/command/context/query/' },
              ]
            },
            { label: '展開: Expand', link: '/command/expand/' },
          ],
        },
        {
          label: '定義済み関数',
          autogenerate: { directory: 'functions' },
        },
      ],
      components: {
        Hero: './src/components/MyHero.astro',
      }
    }),
  ],
  markdown: {
    remarkPlugins: [
      // 改行を <br> に変換する
      remarkBreaks,
    ],
    extendDefaultPlugins: true,
  }
});
