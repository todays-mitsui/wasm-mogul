import { updateContext } from './updateContext.js';

/**
 * @param {{ context: () => string[] }} module
 */
export function initSettings(module) {
  const displayStyle = getDisplayStyle();
  const radios = document.querySelectorAll(`input[name=${KEY_DISPLAY_STYLE}]`);

  for (const radio of radios) {
    if (radio.value === displayStyle) { radio.checked = true; }

    radio.addEventListener('change', function (event) {
      setDisplayStyle(radio.value);
      updateContext(module.context());
    });
  }
}

const KEY_DISPLAY_STYLE = 'tuber_display_style';

export function getDisplayStyle() {
  const strage = window.localStorage;
  return strage.getItem(KEY_DISPLAY_STYLE) ?? 'ECMAScript';
}

function setDisplayStyle(value) {
  const strage = window.localStorage;
  strage.setItem(KEY_DISPLAY_STYLE, value);
}
