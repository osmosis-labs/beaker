import { isNumberParseable } from './';

describe('unit | isNumberParseable', () => {
  it('returns `true` for values parseable number', () => {
    expect(isNumberParseable('-7.5')).toBe(true);
    expect(isNumberParseable(false)).toBe(true);
    expect(isNumberParseable(1892)).toBe(true);
  });

  it('returns `false` for values non parseable to number', () => {
    expect(isNumberParseable('A8sa')).toBe(false);
    expect(isNumberParseable({})).toBe(false);
    expect(isNumberParseable(NaN)).toBe(false);
    expect(isNumberParseable('18L')).toBe(false);
  });
});
