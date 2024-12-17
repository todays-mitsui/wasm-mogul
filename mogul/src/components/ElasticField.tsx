import {
  type JSX,
  type Accessor,
  createRenderEffect,
  splitProps,
} from "solid-js";

interface ElasticFieldProps {
  value: string;
}

export function ElasticField(props: ElasticFieldProps): JSX.Element {
  const [, remainingProps] = splitProps(props, ["value"]);

  return <textarea use:elastic={props.value} {...remainingProps} />;
}

declare module "solid-js" {
  namespace JSX {
    interface Directives {
      elastic: string;
    }
  }
}

function elastic(el: HTMLTextAreaElement, value: Accessor<string>) {
  createRenderEffect(() => {
    el.value = value();
    setTimeout(() => {
      fit(el);
    }, 0);
  });

  el.addEventListener("input", (event) => {
    if (
      event instanceof InputEvent &&
      event.target instanceof HTMLTextAreaElement
    ) {
      fit(event.target);
    }
  });
}

function fit(el: HTMLTextAreaElement) {
  el.style.height = "auto";
  el.style.height = `${el.scrollHeight}px`;
}
