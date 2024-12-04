import type { Meta, StoryObj } from "storybook-solidjs";
import { ConsoleUnitUpdate } from "~/components/Console";

const meta = {
  component: ConsoleUnitUpdate,
  parameters: {
    layout: "fullscreen",
  },
} satisfies Meta<typeof ConsoleUnitUpdate>;

export default meta;
type Story = StoryObj<typeof meta>;

const func = {
  name: "k",
  params: ["x", "y", "z"],
  body: {
    Apply: {
      lhs: {
        Apply: {
          lhs: { Variable: { identifier: "x" } },
          rhs: { Variable: { identifier: "z" } },
        },
      },
      rhs: {
        Apply: {
          lhs: { Variable: { identifier: "y" } },
          rhs: { Variable: { identifier: "z" } },
        },
      },
    },
  },
};

export const LazyK: Story = {
  name: "Lazy_K style",
  args: {
    func,
    displayStyle: "LazyK",
  },
};

export const EcmaScript: Story = {
  name: "ECMAScript style",
  args: {
    func,
    displayStyle: "EcmaScript",
  },
};
