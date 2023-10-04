function main() {
  const form = document.getElementById('form');
  const input = document.getElementById('src');

  import('../pkg/index.js')
    .then(module => {
      console.log('ready to calculate');
      console.log({ lambda_calculus: module.lambda_calculus });
      return module;
    })
    .then(module => {
      form.addEventListener('submit', function (event) {
        event.preventDefault();
        const src = input.value;
        console.log({ src });
        const result = module.lambda_calculus(src, 'ECMAScript');
        console.log({ result });
        input.value = '';
      });
    })
    .catch(err => { console.error(err); });
}

document.addEventListener('DOMContentLoaded', main);
