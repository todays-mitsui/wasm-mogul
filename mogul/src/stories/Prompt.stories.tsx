import type { Meta, StoryObj } from "storybook-solidjs";
import Prompt from "~/components/Prompt";

const meta = {
  component: Prompt,
  parameters: {
    layout: "padded",
  },
} satisfies Meta<typeof Prompt>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: {},
};
