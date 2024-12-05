import { createSignal } from "solid-js";
import type { Meta, StoryObj } from "storybook-solidjs";
import { ConsoleUnitReduceLast } from "~/components/Console";

const meta = {
  component: ConsoleUnitReduceLast,
  parameters: {
    layout: "fullscreen",
  },
} satisfies Meta<typeof ConsoleUnitReduceLast>;

export default meta;
type Story = StoryObj<typeof meta>;

const [reduceResultLazyK] = createSignal(dummyReduceResult().LazyK);
const [reduceResultEcmaScript] = createSignal(dummyReduceResult().EcmaScript);

export const LazyK: Story = {
  name: "Lazy_K style",
  args: {
    formed: dummyFormed().LazyK,
    reduceResult: reduceResultLazyK,
  },
};

export const EcmaScript: Story = {
  name: "ECMAScript style",
  args: {
    formed: dummyFormed().EcmaScript,
    reduceResult: reduceResultEcmaScript,
  },
};

// ========================================================================== //

function dummyFormed() {
  return {
    LazyK: {
      expr: "``````skkikxy",
      reducibleRange: {
        entire: {
          start: 3,
          end: 10,
        },
        callee: {
          start: 6,
          end: 7,
        },
        args: [
          {
            start: 7,
            end: 8,
          },
          {
            start: 8,
            end: 9,
          },
          {
            start: 9,
            end: 10,
          },
        ],
      },
    },
    EcmaScript: {
      expr: "s(k, k, i)(k, x, y)",
      reducibleRange: {
        entire: {
          start: 0,
          end: 10,
        },
        callee: {
          start: 0,
          end: 1,
        },
        args: [
          {
            start: 2,
            end: 3,
          },
          {
            start: 5,
            end: 6,
          },
          {
            start: 8,
            end: 9,
          },
        ],
      },
    },
  };
}

function dummyReduceResult() {
  return {
    LazyK: {
      step: 4,
      formed: {
        expr: "x",
        reducedRange: {
          start: 0,
          end: 1,
        },
        reducibleRange: null,
      },
    },
    EcmaScript: {
      step: 4,
      formed: {
        expr: "x",
        reducedRange: {
          start: 0,
          end: 1,
        },
        reducibleRange: null,
      },
    },
  };
}
