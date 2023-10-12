export function initInput() {
  const input = document.querySelector('#input input');
  const submit = document.querySelector('#input button[type=submit]');

  if (location.search) {
    const param = new URLSearchParams(location.search);
    if (param.has('input')) {
      input.value = param.get('input');
    }
  }

  submit.disabled = !input.value.trim();
  input.addEventListener('input', function (event) {
    submit.disabled = !input.value.trim();
  });

  input.addEventListener('focusin', function (event) {
    input.placeholder = '';
  });
  input.addEventListener('focusout', function (event) {
    input.placeholder = '_';
  });
}
