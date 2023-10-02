document.addEventListener('DOMContentLoaded', function () {
  const container = document.getElementById('container');
  const form = document.getElementById('form');
  const input = document.getElementById('src');

  form.addEventListener('submit', function (event) {
    event.preventDefault();
    const src = input.value;
    console.log({ src });
    lambda_calculus(src);
    input.value = '';
  });

  const observer = new MutationObserver(function (mutations) {
    mutations.forEach(function (m) {
      if (m.addedNodes.length === 0) return;

      container.scrollTo({
        top: container.scrollHeight,
        behavior: 'smooth',
      });
    });
  });
  observer.observe(container, { childList: true });
});
