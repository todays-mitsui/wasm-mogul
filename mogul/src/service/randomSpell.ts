import {
  type DisplayStyle,
  parseExpr,
  renderExpr,
} from "../../../ski3/pkg/index";
import { displayStyle as getDisplayStyle } from "~/signals";
import { createSignal } from "solid-js";

const [spellIndex, setSpellIndex] = createSignal<number>();

export function randomSpell(displayStyle?: DisplayStyle): string {
  const spell = getRandomSpell();

  const expr = renderExpr(
    parseExpr(spell.expr),
    displayStyle ?? getDisplayStyle(),
  );
  return spell.commandPrefix ? `${spell.commandPrefix} ${expr}` : expr;
}

function getRandomSpell(): Spell {
  let i;
  do {
    i = Math.floor(Math.random() * spells.length);
  } while (spellIndex() === i);

  setSpellIndex(i);

  return spells[i];
}

interface Spell {
  commandPrefix: string | null;
  expr: string;
}

const spells: Spell[] = [
  {
    commandPrefix: null,
    expr: "(x => x)(3)",
  },
  {
    commandPrefix: "",
    expr: "EQ(ADD(2, 3), 5)",
  },
  {
    commandPrefix: "",
    expr: "EQ(SUB(7, 3), 2)",
  },
  {
    commandPrefix: "",
    expr: "MUL(ADD(1, 3), 2)(:f, :x)",
  },
  {
    commandPrefix: "",
    expr: "NOT(FALSE)",
  },
  {
    commandPrefix: "",
    expr: "AND(TRUE, NOT(FALSE))",
  },
  {
    commandPrefix: "?",
    expr: "3",
  },
  {
    commandPrefix: "?",
    expr: "s",
  },
  {
    commandPrefix: "?",
    expr: "NOT",
  },
  {
    commandPrefix: "~",
    expr: "ADD(1, 2)",
  },
  {
    commandPrefix: "~~",
    expr: "f => x => y => f(y, x)",
  },
  {
    commandPrefix: "~~~~",
    expr: "ADD(1, 2)",
  },
];
