export function initInput() {
  const input = document.querySelector('#input input');
  const submit = document.querySelector('#input button[type=submit]');

  submit.disabled = !input.value.trim();
  input.addEventListener('input', function (event) {
    submit.disabled = !input.value.trim();
  });

  input.addEventListener('focusin', function (event) {
    console.log('focus');
    input.placeholder = '';
  });
  input.addEventListener('focusout', function (event) {
    console.log('blur');
    input.placeholder = '_';
  });
}
