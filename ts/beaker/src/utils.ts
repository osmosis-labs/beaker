const id = (x: any) => x;

export const mapObject = (o: Object, f: Function, g: Function): Object =>
  Object.fromEntries(Object.entries(o).map(([k, v]) => [f(k), g(v)]));

export const mapValues = (o: Object, g: Function) => mapObject(o, id, g);

export const extendWith = (properties: Object) => (context: Object) => {
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
