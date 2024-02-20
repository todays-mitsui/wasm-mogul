import { updateContext } from './updateContext.js';

/**
 * @param {{ Context: Context }} module
 */
export function initSettings(module) {
  const displayStyle = getDisplayStyle();
  const radios = document.querySelectorAll(`input[name=${KEY_DISPLAY_STYLE}]`);

  for (const radio of radios) {
    if (radio.value === displayStyle) { radio.checked = true; }

    radio.addEventListener('change', function (event) {
      setDisplayStyle(radio.value);
      updateContext(module);
    });
  }

  const resetButton = document.querySelector(`button[name=${KEY_RESET_CONTEXT}]`);
  if (resetButton) {
    resetButton.addEventListener('click', resetContext(module));
  }

  const clearButton = document.querySelector(`button[name=${KEY_CLEAR_CONTEXT}]`);
  if (clearButton) {
    clearButton.addEventListener('click', clearContext(module));
  }
}

const KEY_DISPLAY_STYLE = 'tuber_display_style';
const KEY_RESET_CONTEXT = 'tuber_reset_context';
const KEY_CLEAR_CONTEXT = 'tuber_clear_context';

export function getDisplayStyle() {
  const strage = window.localStorage;
  return strage.getItem(KEY_DISPLAY_STYLE) ?? 'ECMAScript';
}

function setDisplayStyle(value) {
  const strage = window.localStorage;
  strage.setItem(KEY_DISPLAY_STYLE, value);
}

function resetContext(module) {
  return () => {
    if (confirm('Context が初期状態に戻されます。よろしいですか？')) {
      gtag('event', 'click', { event_category: 'control', event_label: 'resetContext', execute: 1 });
      const context = new module.Context();
      context.reset();
      updateContext(module);
      alert('Context が初期状態に戻されました');
    } else {
      gtag('event', 'click', { event_category: 'control', event_label: 'resetContext', execute: 0 });
    }
  }
}

function clearContext(module) {
  return () => {
    if (confirm('Context に登録された全ての Function が削除されます。よろしいですか？')) {
      gtag('event', 'click', { event_category: 'control', event_label: 'clearContext', execute: 1 });
      const context = new module.Context();
      context.deleteAll();
      updateContext(module);
      alert('Context に登録された全ての Function が削除されました');
    } else {
      gtag('event', 'click', { event_category: 'control', event_label: 'clearContext', execute: 0 });
    }
  }
}
