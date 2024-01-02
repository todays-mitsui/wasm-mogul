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
            { label: '式: Expression', link: '/syntax/expression/' },
            { label: '変数: Variable', link: '/syntax/expression/#変数-variable' },
            { label: 'シンボル: Symbol', link: '/syntax/expression/#シンボル-symbol' },
            { label: '関数適用: Application', link: '/syntax/expression/#関数適用-application' },
            { label: 'ラムダ抽象: Lambda', link: '/syntax/expression/#ラムダ抽象-lambda' },
          ],
        },
        { label: '名前付き関数とコンテキスト', link: '/function_and_context/' },
        {
          label: 'コマンド: Command',
          items: [
            { label: '簡約: Reduce', link: '/command/reduce/' },
            { label: '関数定義: Define', link: '/command/define/' },
            {
              label: 'コンテキスト: Context',
              items: [
                { label: '一覧: List', link: '/command/context_list/' },
                { label: '問い合わせ: Query', link: '/command/context_query/' },
              ]
            },
            { label: '展開: Expand', link: '/command/expand/' },
          ],
        },
        {
          label: '定義済み関数',
          items: [
            { label: '定義済み関数', link: '/functions/' },
            { label: '基本的なコンビネータ', link: '/functions/basic_combinator/' },
            { label: '真理値', link: '/functions/truth_value/' },
            { label: '自然数', link: '/functions/number/' },
            { label: '比較演算', link: '/functions/compare/' },
            { label: 'タプルとリスト', link: '/functions/tuple_and_list/' },
            { label: '不動点コンビネータ', link: '/functions/fixed_point_combinator/' },
          ],
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
