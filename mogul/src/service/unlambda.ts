import {
  type Expr,
  type DisplayStyle,
  expand,
  unlambdaRecursive,
  unlambdaRecursive_,
  unlambdaIota,
  renderExpr as render,
} from "../../../ski3/pkg/index";
import { context, displayStyle as getDisplayStyle } from "~/signals";

export function unlambda(level: number, expr: Expr): Expr {
  switch (level) {
    case 1:
      return expand(context(), expr);
    case 2:
      return unlambdaRecursive(context(), expr);
    case 3:
      return unlambdaRecursive_(context(), expr);
    case 4:
      return unlambdaIota(context(), expr);
    default:
      throw new Error("Invalid level");
  }
}

export function renderExpr(expr: Expr, displayStyle?: DisplayStyle): string {
  return render(expr, displayStyle ?? getDisplayStyle());
}
