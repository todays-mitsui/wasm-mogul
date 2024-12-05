import type { Meta, StoryObj } from "storybook-solidjs";
import { ConsoleUnitQueryDefined } from "~/components/Console";

const meta = {
  component: ConsoleUnitQueryDefined,
  parameters: {
    layout: "fullscreen",
  },
} satisfies Meta<typeof ConsoleUnitQueryDefined>;

export default meta;
type Story = StoryObj<typeof meta>;

const func = {
  name: "k",
  params: ["x", "y"],
  body: { Variable: { identifier: "x" } },
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
