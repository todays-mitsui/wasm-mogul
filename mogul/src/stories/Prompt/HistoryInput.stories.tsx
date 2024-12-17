import type { Meta, StoryObj } from "storybook-solidjs";
import { HistoryInput } from "~/components/Prompt";

const meta = {
  component: HistoryInput,
  parameters: {
    layout: "padded",
  },
} satisfies Meta<typeof HistoryInput>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: {
    value: "",
    history: ["1st", "2nd", "3rd", "4th", "5th"],
  },
};
