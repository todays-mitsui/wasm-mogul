import { createSignal } from "solid-js";
import type { Meta, StoryObj } from "storybook-solidjs";
import { ElasticField } from "~/components/ElasticField";

const meta = {
  component: ElasticField,
  parameters: {
    layout: "padded",
  },
} satisfies Meta<typeof ElasticField>;

export default meta;
type Story = StoryObj<typeof meta>;

const [value, setValue] = createSignal(`foo
bar
baz`);

export const Default: Story = {
  args: {
    value: value(),
    onInput: (event) => {
      setValue(event.target.value);
      console.log({ value: value() });
    },
  },
};
