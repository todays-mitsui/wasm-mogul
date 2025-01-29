import type { Meta, StoryObj } from "storybook-solidjs";
import { ConsoleUnitParseError } from "~/components/Console";

const meta = {
  component: ConsoleUnitParseError,
  parameters: {
    layout: "fullscreen",
  },
} satisfies Meta<typeof ConsoleUnitParseError>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: {
    message: "Parse error Unexpected `(` Expected whitespace or end of input",
  },
};
