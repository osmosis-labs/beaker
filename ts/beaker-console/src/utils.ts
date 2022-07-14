/* eslint-disable */

const id = <T>(x: T) => x;

export const mapObject = (
  o: Record<string, unknown>,
  f: Function,
  g: Function,
): Record<string, unknown> =>
  Object.fromEntries(Object.entries(o).map(([k, v]) => [f(k), g(v)]));

export const mapKV = (
  o: Record<string, unknown>,
  f: Function,
): Record<string, unknown> =>
  Object.fromEntries(Object.entries(o).map(([k, v]) => f(k, v)));

export const mapValues = (o: Record<string, unknown>, g: Function) =>
  mapObject(o, id, g);

export const extendWith =
  (properties: Record<string, unknown>) =>
  (context: Record<string, unknown>) => {
    Object.entries(properties).forEach(([k, v]) => {
      // @ts-ignore
      context[k] = v;
      // Object.defineProperty(context, k, {
      //   configurable: true,
      //   enumerable: true,
      //   value: v,
      // });
    });
  };
