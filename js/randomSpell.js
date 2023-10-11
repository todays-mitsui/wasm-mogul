import { getDisplayStyle } from './settings.js';

const spells = {
  'ECMAScript': [
    '(x => x)(3)',
    'EQ(ADD(2, 3), 5)',
    'EQ(SUB(7, 3), 2)',
    'MUL(ADD(1, 3), 2)(:f, :x)',
    'AND(TRUE, NOT(FALSE))',
    '? s',
    '? NOT',
    '~ f => x => y => f(y, x)',
    '~~ ADD(1, 2)',
    '~~~~ ADD(1, 2)',
  ],
  'Lazy_K': [
    '`λx.x3',
    '``EQ``ADD 2 3 5',
    '``EQ``SUB 7 3 2',
    '`` ``MUL``ADD 1 3 2 :f:x',
    '``AND TRUE `NOT FALSE',
    '? s',
    '? NOT',
    '~ λf.λx.λy.``fyx',
    '~~ ``ADD 1 2',
    '~~~~ ``ADD 1 2',
  ],
};

function getRandomSpell(displayStyle) {
  const spellList = spells[displayStyle];
  const index = Math.floor(Math.random() * spellList.length);
  return spellList[index];
}

export function initRandomSpell() {
  const input = document.querySelector('#input input');
  const button = document.querySelector('#input button[type=button]');

  button.addEventListener('click', function (event) {
    const displayStyle = getDisplayStyle();

    let spell;
    do {
      spell = getRandomSpell(displayStyle);
    } while (input.value === spell);

    input.value = spell;

    input.dispatchEvent(new Event('input'));
  });
}
