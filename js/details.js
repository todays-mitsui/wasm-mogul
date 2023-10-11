export function initDetails() {
  const details = document.querySelectorAll('details[name=tool]');

  const onClick = function (event) {
    const detail = event.target.closest('details');
    if (detail.open) { return; }
    for (const detail of details) {
      detail.open = false;
    }
  };

  for (const detail of details) {
    detail.addEventListener('click', onClick);
  }

  const outputBox = document.querySelector('#output');

  outputBox.addEventListener('click', function (event) {
    for (const detail of details) {
      detail.open = false;
    }
  });
}
