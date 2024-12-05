import type { Meta, StoryObj } from "storybook-solidjs";
import { ConsoleUnitQueryUndefined } from "~/components/Console";

const meta = {
  component: ConsoleUnitQueryUndefined,
  parameters: {
    layout: "fullscreen",
  },
} satisfies Meta<typeof ConsoleUnitQueryUndefined>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: {
    identifier: "u",
  },
};
