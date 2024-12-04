import { createSignal } from "solid-js";
import type { Meta, StoryObj } from "storybook-solidjs";
import { ConsoleUnitReduce } from "~/components/Console";

const meta = {
  component: ConsoleUnitReduce,
  parameters: {
    layout: "fullscreen",
  },
} satisfies Meta<typeof ConsoleUnitReduce>;

export default meta;
type Story = StoryObj<typeof meta>;

const [reduceResultsLazyK] = createSignal(dummyReduceResults().LazyK);
const [reduceResultsEcmaScript] = createSignal(dummyReduceResults().EcmaScript);

export const LazyK: Story = {
  name: "Lazy_K style",
  args: {
    formed: dummyFormed().LazyK,
    reduceResults: reduceResultsLazyK,
  },
};

export const EcmaScript: Story = {
  name: "ECMAScript style",
  args: {
    formed: dummyFormed().EcmaScript,
    reduceResults: reduceResultsEcmaScript,
  },
};

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

function dummyReduceResults() {
  return {
    LazyK: [
      {
        step: 1,
        formed: {
          expr: "`````ki`kikxy",
          reducedRange: {
            start: 3,
            end: 10,
          },
          reducibleRange: {
            entire: {
              start: 3,
              end: 10,
            },
            callee: {
              start: 5,
              end: 6,
            },
            args: [
              {
                start: 6,
                end: 7,
              },
              {
                start: 7,
                end: 10,
              },
            ],
          },
        },
      },
      {
        step: 2,
        formed: {
          expr: "```ikxy",
          reducedRange: {
            start: 3,
            end: 4,
          },
          reducibleRange: {
            entire: {
              start: 2,
              end: 5,
            },
            callee: {
              start: 3,
              end: 4,
            },
            args: [
              {
                start: 4,
                end: 5,
              },
            ],
          },
        },
      },
      {
        step: 3,
        formed: {
          expr: "``kxy",
          reducedRange: {
            start: 2,
            end: 3,
          },
          reducibleRange: {
            entire: {
              start: 0,
              end: 5,
            },
            callee: {
              start: 2,
              end: 3,
            },
            args: [
              {
                start: 3,
                end: 4,
              },
              {
                start: 4,
                end: 9,
              },
            ],
          },
        },
      },
      {
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
    ],
    EcmaScript: [
      {
        step: 1,
        formed: {
          expr: "k(i, k(i))(k, x, y)",
          reducedRange: {
            start: 0,
            end: 10,
          },
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
                end: 9,
              },
            ],
          },
        },
      },
      {
        step: 2,
        formed: {
          expr: "i(k)(x, y)",
          reducedRange: {
            start: 0,
            end: 1,
          },
          reducibleRange: {
            entire: {
              start: 0,
              end: 4,
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
            ],
          },
        },
      },
      {
        step: 3,
        formed: {
          expr: "k(x, y)",
          reducedRange: {
            start: 0,
            end: 1,
          },
          reducibleRange: {
            entire: {
              start: 0,
              end: 7,
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
            ],
          },
        },
      },
      {
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
    ],
  };
}
