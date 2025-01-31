import { aliases as getAliases, setAliases, displayStyle } from "~/signals";
import { type Aliases, type Expr, renderExpr } from "../../../ski3/pkg/index";
export { Aliases };

export function addAlias(expr: Expr) {
  setAliases((prev) => {
    const newAliases: Aliases = { _: expr };
    if (prev._) newAliases._0 = prev._;
    if (prev._0) newAliases._1 = prev._0;
    if (prev._1) newAliases._2 = prev._1;
    if (prev._2) newAliases._3 = prev._2;
    if (prev._3) newAliases._4 = prev._3;
    if (prev._4) newAliases._5 = prev._4;
    if (prev._5) newAliases._6 = prev._5;
    if (prev._6) newAliases._7 = prev._6;
    if (prev._7) newAliases._8 = prev._7;
    if (prev._8) newAliases._9 = prev._8;
    return newAliases;
  });
}

export function aliases(): string[] {
  return Object.entries(getAliases())
    .toSorted(([a], [b]) => a.localeCompare(b))
    .map(([name, expr]) => `${name} = ${renderExpr(expr, displayStyle())}`);
}
